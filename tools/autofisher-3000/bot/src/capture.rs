//! Frame capture + detection loop.
//!
//! Frames are pulled from the portal's PipeWire stream through OpenCV's
//! `VideoCapture` (GStreamer backend — Fedora's OpenCV is built with it), which
//! hands us `Mat`s directly. Each frame is run through the active [`Detector`]
//! backend and the resulting bobber centers are published to the controller via a
//! watch channel. Runs on a dedicated blocking thread.

use crate::detect::{Detector, LureMatcher};
use crate::frame::Frame;
use crate::state::Point;
use anyhow::{Context, Result};
use opencv::core::Scalar;
use opencv::prelude::*;
use opencv::{highgui, imgproc, videoio};
use std::os::fd::RawFd;
use std::time::{Duration, Instant};
use tokio::sync::watch;

/// Latest detection snapshot handed to the controller.
#[derive(Clone, Default)]
pub struct CaptureState {
    pub center: Point,
    pub targets: Vec<Point>,
    pub width: u32,
    pub height: u32,
    /// `Some(true/false)` when lure detection is enabled, `None` when it's off.
    pub lure_present: Option<bool>,
}

const WINDOW: &str = "autofisher-3000";

/// Build the GStreamer pipeline string for OpenCV's `VideoCapture`.
///
/// `videorate` caps the stream to `target_fps` *before* the BGR readback, so the
/// ~60→`target_fps` reduction also cuts the per-frame GPU→system-memory copy at the
/// source (not just the detection work downstream).
pub fn pipeline(fd: RawFd, node_id: u32, target_fps: u32) -> String {
    format!(
        "pipewiresrc fd={fd} path={node_id} ! videoconvert ! videorate ! \
         video/x-raw,format=BGR,framerate={target_fps}/1 ! \
         appsink drop=true max-buffers=2 sync=false"
    )
}

fn open(pipeline: &str) -> Result<videoio::VideoCapture> {
    let cap = videoio::VideoCapture::from_file(pipeline, videoio::CAP_GSTREAMER)
        .context("opening the PipeWire/GStreamer capture pipeline")?;
    anyhow::ensure!(
        cap.is_opened()?,
        "capture pipeline did not open (is the portal stream live?)"
    );
    Ok(cap)
}

/// Capture + detect until the window is closed (`q`, debug) or the channel drops.
pub fn run(
    pipeline: String,
    mut detector: Box<dyn Detector + Send>,
    lure: Option<LureMatcher>,
    tx: watch::Sender<CaptureState>,
    debug: bool,
) -> Result<()> {
    let mut cap = open(&pipeline)?;
    if debug {
        highgui::named_window(WINDOW, highgui::WINDOW_NORMAL)?;
    }

    let mut frame = Mat::default();
    // warm up the stream: the first frames after a PipeWire stream starts can be
    // black/partial until format negotiation settles, which would mis-read the lure
    for _ in 0..15 {
        let _ = cap.read(&mut frame);
    }

    let mut had_targets = false;
    let mut lure_present: Option<bool> = None;
    let mut last_lure_check = Instant::now() - Duration::from_secs(10);
    loop {
        if !cap.read(&mut frame)? || frame.empty() {
            continue;
        }
        let width = frame.cols() as u32;
        let height = frame.rows() as u32;
        // bridge the Mat into an owned Frame so the detector backend stays
        // opencv-agnostic (cheap at the capped framerate)
        let snapshot = Frame::new(frame.data_bytes()?.to_vec(), width, height);
        let targets = detector.detect(&snapshot)?;

        // log on the rising edge so a cast bobber shows up in the console
        if !targets.is_empty() && !had_targets {
            tracing::info!(count = targets.len(), "bobber(s) detected");
        }
        had_targets = !targets.is_empty();

        // the lure buff changes slowly (~10-min duration), so check it every couple
        // of seconds to keep the cost off the detection hot path
        if let Some(ref matcher) = lure {
            if last_lure_check.elapsed() >= Duration::from_secs(2) {
                last_lure_check = Instant::now();
                match matcher.present(&frame) {
                    Ok(p) => {
                        if lure_present != Some(p) {
                            tracing::info!(present = p, "lure status");
                        }
                        lure_present = Some(p);
                    }
                    Err(e) => tracing::warn!(error = %e, "lure match failed"),
                }
            }
        }

        if debug {
            // mark each detected center (the trait returns points, not boxes)
            for p in &targets {
                imgproc::circle(
                    &mut frame,
                    opencv::core::Point::new(p.x as i32, p.y as i32),
                    12,
                    Scalar::new(0.0, 255.0, 0.0, 0.0),
                    2,
                    imgproc::LINE_8,
                    0,
                )?;
            }
            highgui::imshow(WINDOW, &frame)?;
            if highgui::wait_key(1)? == 'q' as i32 {
                break;
            }
        }

        // a closed channel (controller gone) means it's time to stop
        if tx
            .send(CaptureState {
                center: Point {
                    x: width as f64 / 2.0,
                    y: height as f64 / 2.0,
                },
                targets,
                width,
                height,
                lure_present,
            })
            .is_err()
        {
            break;
        }
    }
    Ok(())
}

/// Grab a single frame and write it to `path` (for cropping a lure icon).
pub fn grab_frame(pipeline: &str, path: &str) -> Result<()> {
    let mut cap = open(pipeline)?;
    let mut frame = Mat::default();
    // discard a few frames to let the stream warm up to real content
    for _ in 0..15 {
        cap.read(&mut frame)?;
    }
    anyhow::ensure!(!frame.empty(), "no frame captured from the stream");
    opencv::imgcodecs::imwrite_def(path, &frame)?;
    Ok(())
}
