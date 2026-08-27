//! Imperative shell: parse args, open the portal, run capture+detection, and
//! (Phase 4) drive the state machine's actions through portal input.

use anyhow::{Context, Result};
use autofisher::capture::{self, CaptureState};
use autofisher::config::{Config, DetectorBackend};
use autofisher::detect::{CascadeDetector, Detector, LureMatcher};
use autofisher::nn::NnDetector;
use autofisher::portal::PortalSession;
use autofisher::state::{step, Action, FishingState, StepCtx};
use clap::Parser;
use opencv::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::watch;
use tokio::time;

#[derive(Parser)]
#[command(
    name = "autofisher-3000",
    about = "World of Warcraft fishing bot (Wayland-native)"
)]
struct Args {
    /// Show a live window with detections drawn (press `q` to quit). Needs a
    /// GUI-enabled OpenCV — available in the devcontainer, not in the Flatpak.
    #[arg(long)]
    debug: bool,
    /// Capture one frame to PATH and exit — use it to crop a lure-buff icon.
    #[arg(long, value_name = "PATH")]
    grab_frame: Option<PathBuf>,
    /// Cascade model to use (overrides the default).
    #[arg(long, value_name = "PATH")]
    model: Option<PathBuf>,
    /// Offline: run the lure matcher against a saved frame (PNG) and print the score.
    #[arg(long, value_name = "PATH")]
    check_lure: Option<PathBuf>,
}

/// Resolve a possibly-relative resource path against the first asset root that
/// actually contains it.
///
/// Roots are tried in order: `$AUTOFISHER_ASSET_DIR`, then
/// `<exe dir>/../share/autofisher-3000` (the installed layout — `/app/share/...`
/// inside the Flatpak), then the crate root (`cargo run` / `cargo test` from the
/// dev tree). If none of them has the file, fall back to the crate root so the
/// caller's "not found" error still names a sensible path.
fn resolve(p: &Path) -> PathBuf {
    if p.is_absolute() {
        return p.to_path_buf();
    }
    asset_roots()
        .into_iter()
        .map(|root| root.join(p))
        .find(|candidate| candidate.exists())
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(p))
}

/// Candidate directories holding `models/` and `icons/`, most specific first.
fn asset_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    // Escape hatch: point a packaged build at a hand-retrained model without a rebuild.
    if let Some(dir) = std::env::var_os("AUTOFISHER_ASSET_DIR") {
        roots.push(PathBuf::from(dir));
    }
    // Installed layout: /app/bin/autofisher-3000 -> /app/share/autofisher-3000
    let installed = std::env::current_exe()
        .ok()
        .and_then(|exe| Some(exe.parent()?.parent()?.join("share/autofisher-3000")));
    roots.extend(installed);
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    roots
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();
    let cfg = Config::default();

    // offline lure check — no portal needed
    if let Some(frame) = args.check_lure {
        return check_lure(&cfg, &frame);
    }

    let portal = PortalSession::open()
        .await
        .context("opening the screen-share / remote-control portal")?;
    tracing::info!(
        node = portal.node_id,
        width = portal.width,
        height = portal.height,
        "portal stream ready"
    );

    let pipeline = capture::pipeline(portal.pipewire_fd(), portal.node_id, cfg.target_fps);

    if let Some(path) = args.grab_frame {
        let path = path.to_string_lossy().into_owned();
        capture::grab_frame(&pipeline, &path)?;
        tracing::info!(path = %path, "saved a frame");
        return Ok(());
    }

    let detector: Box<dyn Detector + Send> = match cfg.backend {
        DetectorBackend::Cascade => {
            let model = args.model.unwrap_or_else(|| PathBuf::from(&cfg.model));
            Box::new(
                CascadeDetector::load(resolve(&model), cfg.min_neighbors)
                    .context("loading the cascade model")?,
            )
        }
        DetectorBackend::Nn => Box::new(
            NnDetector::load(
                resolve(Path::new(&cfg.nn_model)),
                cfg.nn_input_size,
                cfg.nn_conf_threshold,
                cfg.nn_iou_threshold,
                cfg.roi,
            )
            .context("loading the ONNX model")?,
        ),
    };
    let lure = build_lure_matcher(&cfg)?;

    // the controller flips this on after each cast so the capture thread re-reads the
    // lure once per cycle (the buff lasts ~10 min); starts true for the first cast.
    let check_lure = Arc::new(AtomicBool::new(true));
    let (tx, rx) = watch::channel(CaptureState::default());
    let debug = args.debug;
    let capture_check_lure = Arc::clone(&check_lure);
    let capture_thread = std::thread::spawn(move || {
        capture::run(pipeline, detector, lure, capture_check_lure, tx, debug)
    });

    tracing::info!("fishing — Ctrl-C to stop (or `q` in the debug window)");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => tracing::info!("stopping (Ctrl-C)"),
        res = run_controller(rx, &portal, &cfg, &check_lure) => res?,
        res = wait_for_thread(capture_thread) => res?,
    }

    drop(portal);
    Ok(())
}

/// Drive the pure state machine: read the latest detections, `step`, and execute
/// the resulting actions through portal input.
async fn run_controller(
    rx: watch::Receiver<CaptureState>,
    portal: &PortalSession,
    cfg: &Config,
    check_lure: &AtomicBool,
) -> Result<()> {
    let mut state = FishingState::new(Instant::now(), cfg);
    let mut ticker = time::interval(Duration::from_millis(50));
    loop {
        ticker.tick().await;
        let snap = rx.borrow().clone();
        let was_confirming = matches!(
            state,
            FishingState::Searching {
                candidate: Some(_),
                ..
            }
        );
        let (next, actions) = step(
            state,
            &StepCtx {
                now: Instant::now(),
                targets: &snap.targets,
                center: snap.center,
                lure_present: snap.lure_present,
                cfg,
            },
        );
        // a streak that lapses without reeling = the bobber flickered out before it
        // could be confirmed (a missed bite) — surface it so the gate can be tuned
        let reeled = actions.iter().any(|a| matches!(a, Action::RightClick));
        let still_confirming = matches!(
            next,
            FishingState::Searching {
                candidate: Some(_),
                ..
            }
        );
        if was_confirming && !still_confirming && !reeled {
            tracing::warn!(
                "bobber lost before confirm — missed bite (lower stability_ms / raise flicker_ms)"
            );
        }
        // every cast lands in Settling — entering it means we just cast, so ask the
        // capture thread to re-read the lure once for the next cycle's cast decision.
        let cast = matches!(next, FishingState::Settling { .. })
            && !matches!(state, FishingState::Settling { .. });
        if cast {
            check_lure.store(true, Ordering::Relaxed);
        }
        state = next;
        for action in actions {
            match action {
                Action::PressKey(code) => {
                    tracing::info!(key = code, "press");
                    portal.press_key(code as i32).await?;
                }
                Action::MoveTo(p) => {
                    portal.move_to(p.x, p.y).await?;
                    // let the cursor settle before the click that follows
                    time::sleep(Duration::from_millis(250)).await;
                }
                Action::RightClick => {
                    tracing::info!("reel in");
                    portal.right_click().await?;
                }
            }
        }
    }
}

/// Offline check: score the lure template against a saved frame and print it.
fn check_lure(cfg: &Config, frame_path: &Path) -> Result<()> {
    let icon = cfg
        .lure_icon
        .as_ref()
        .context("no lure_icon configured in the default config")?;
    let matcher = LureMatcher::load(resolve(Path::new(icon)), cfg.lure_threshold)
        .context("loading the lure icon")?;
    let file = frame_path
        .to_str()
        .context("frame path is not valid UTF-8")?;
    let frame = opencv::imgcodecs::imread_def(file)
        .with_context(|| format!("reading frame {}", frame_path.display()))?;
    anyhow::ensure!(
        !frame.empty(),
        "could not read frame: {}",
        frame_path.display()
    );
    let score = matcher.score(&frame)?;
    println!(
        "lure match score {score:.3} (threshold {:.2}) -> {}",
        cfg.lure_threshold,
        if score >= cfg.lure_threshold {
            "PRESENT"
        } else {
            "absent"
        }
    );
    Ok(())
}

/// Build the lure matcher when lure detection is on and an icon is configured.
fn build_lure_matcher(cfg: &Config) -> Result<Option<LureMatcher>> {
    if cfg.skip_lure {
        return Ok(None);
    }
    match &cfg.lure_icon {
        Some(icon) => Ok(Some(
            LureMatcher::load(resolve(Path::new(icon)), cfg.lure_threshold)
                .context("loading the lure icon")?,
        )),
        None => {
            tracing::warn!("skip_lure is off but no lure_icon is set — lure disabled");
            Ok(None)
        }
    }
}

/// Await a blocking capture thread without busy-waiting.
async fn wait_for_thread(handle: std::thread::JoinHandle<Result<()>>) -> Result<()> {
    tokio::task::spawn_blocking(move || handle.join())
        .await
        .context("capture thread panicked")?
        .map_err(|_| anyhow::anyhow!("capture thread panicked"))?
}
