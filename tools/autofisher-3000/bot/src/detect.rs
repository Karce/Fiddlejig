//! Object-detection backends.
//!
//! Today: an OpenCV Haar cascade over the trained `.xml` models. The [`Detector`]
//! trait is the seam for swapping in a learned model (ONNX / vision LLM) later —
//! see the README roadmap.

use crate::frame::Frame;
use crate::state::Point;
use anyhow::Context;
use opencv::core::{Rect, Size, ToInputArray, Vector};
use opencv::objdetect::CascadeClassifier;
use opencv::prelude::*;
use std::path::Path;

/// Finds bobber click-points (centers) in a frame.
pub trait Detector {
    fn detect(&mut self, frame: &Frame) -> anyhow::Result<Vec<Point>>;
}

/// OpenCV Haar cascade detector.
pub struct CascadeDetector {
    cascade: CascadeClassifier,
    scale_factor: f64,
    min_neighbors: i32,
}

impl CascadeDetector {
    /// Load a trained cascade `.xml`. `min_neighbors` trades recall for precision —
    /// higher drops more weak detections (false positives).
    pub fn load(model_path: impl AsRef<Path>, min_neighbors: i32) -> anyhow::Result<Self> {
        let path = model_path.as_ref();
        let file = path.to_str().context("model path is not valid UTF-8")?;
        let cascade = CascadeClassifier::new(file)
            .with_context(|| format!("loading cascade model {}", path.display()))?;
        anyhow::ensure!(
            !cascade.empty()?,
            "cascade model is empty or not a valid classifier: {}",
            path.display()
        );
        Ok(Self {
            cascade,
            scale_factor: 1.1,
            min_neighbors,
        })
    }

    /// Detect raw bounding boxes in a BGR image (the cascade runs on grayscale).
    pub fn detect_rects(&mut self, bgr: &impl ToInputArray) -> anyhow::Result<Vec<Rect>> {
        let mut gray = Mat::default();
        opencv::imgproc::cvt_color_def(bgr, &mut gray, opencv::imgproc::COLOR_BGR2GRAY)?;

        let mut objects: Vector<Rect> = Vector::new();
        self.cascade.detect_multi_scale(
            &gray,
            &mut objects,
            self.scale_factor,
            self.min_neighbors,
            0,
            Size::new(0, 0),
            Size::new(0, 0),
        )?;
        Ok(objects.to_vec())
    }

    /// Detect bobber centers in a BGR image.
    pub fn detect_mat(&mut self, bgr: &impl ToInputArray) -> anyhow::Result<Vec<Point>> {
        Ok(self.detect_rects(bgr)?.iter().map(rect_center).collect())
    }
}

/// Center point of a detection box.
pub fn rect_center(r: &Rect) -> Point {
    Point {
        x: r.x as f64 + r.width as f64 / 2.0,
        y: r.y as f64 + r.height as f64 / 2.0,
    }
}

impl Detector for CascadeDetector {
    fn detect(&mut self, frame: &Frame) -> anyhow::Result<Vec<Point>> {
        self.detect_mat(&frame.to_mat()?)
    }
}

/// Detects whether the lure buff icon is on screen, by template-matching a cropped
/// icon against the frame. Drives the decision to re-apply the lure.
pub struct LureMatcher {
    template: Mat,
    threshold: f64,
}

impl LureMatcher {
    /// Load the lure-buff icon template (a cropped screenshot of the buff icon).
    pub fn load(icon_path: impl AsRef<Path>, threshold: f64) -> anyhow::Result<Self> {
        let path = icon_path.as_ref();
        let file = path.to_str().context("lure icon path is not valid UTF-8")?;
        let template = opencv::imgcodecs::imread_def(file)
            .with_context(|| format!("loading lure icon {}", path.display()))?;
        anyhow::ensure!(
            !template.empty(),
            "lure icon is empty or unreadable: {}",
            path.display()
        );
        Ok(Self {
            template,
            threshold,
        })
    }

    /// Best template-match score (0–1) for the lure icon anywhere in the frame.
    pub fn score(&self, bgr: &impl ToInputArray) -> anyhow::Result<f64> {
        let mut result = Mat::default();
        opencv::imgproc::match_template_def(
            bgr,
            &self.template,
            &mut result,
            opencv::imgproc::TM_CCOEFF_NORMED,
        )?;
        let mut max_val = 0.0;
        opencv::core::min_max_loc(
            &result,
            None,
            Some(&mut max_val),
            None,
            None,
            &Mat::default(),
        )?;
        Ok(max_val)
    }

    /// True if the lure buff is found above the confidence threshold.
    pub fn present(&self, bgr: &impl ToInputArray) -> anyhow::Result<bool> {
        Ok(self.score(bgr)? >= self.threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_model() -> CascadeDetector {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/models/bobber_z4_10v3.xml");
        CascadeDetector::load(p, 5).expect("load default cascade model")
    }

    #[test]
    fn loads_model_and_runs_on_blank_frame() {
        // Smoke test: the committed model loads and detection runs end-to-end on a
        // synthetic black frame (which contains no bobber → no detections). Avoids
        // committing real screenshots to this public repo.
        let mut det = default_model();
        let frame = Frame::new(vec![0u8; 200 * 200 * 3], 200, 200);
        let points = det.detect(&frame).expect("detection runs");
        assert!(points.is_empty(), "a blank frame should yield no bobbers");
    }

    /// Runs the default cascade over the local (gitignored) training corpus to
    /// confirm it genuinely finds bobbers — not just that detection executes.
    /// Ignored by default (the corpus isn't committed); run with:
    ///   cargo test -- --ignored --nocapture
    #[test]
    #[ignore = "needs the local, gitignored training corpus"]
    fn detects_bobbers_in_local_corpus() {
        use opencv::imgcodecs;

        let root =
            std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../training/positive"));
        if !root.exists() {
            eprintln!("corpus not present at {}; skipping", root.display());
            return;
        }

        let mut det = default_model();
        let (mut images, mut with_hit, mut total) = (0u32, 0u32, 0u32);
        for zone in std::fs::read_dir(root).unwrap().flatten() {
            if !zone.path().is_dir() {
                continue;
            }
            for entry in std::fs::read_dir(zone.path()).unwrap().flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) != Some("jpg") {
                    continue;
                }
                let mat = imgcodecs::imread_def(p.to_str().unwrap()).unwrap();
                if mat.empty() {
                    continue;
                }
                let pts = det.detect_mat(&mat).unwrap();
                images += 1;
                total += pts.len() as u32;
                if !pts.is_empty() {
                    with_hit += 1;
                }
            }
        }

        eprintln!(
            "corpus: {images} positive images, {with_hit} with >=1 detection, {total} total detections"
        );
        assert!(images > 0, "no corpus images found");
        assert!(
            with_hit > 0,
            "detector found zero bobbers across the entire positive corpus — likely broken"
        );
    }
}
