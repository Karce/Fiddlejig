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

/// Detects whether the lure buff icon is on screen, by multi-scale template-matching a
/// canonical icon against the frame. Drives the decision to re-apply the lure.
///
/// Resolution-independent: the buff icon's on-screen pixel size scales with the game's
/// render resolution, so a single fixed-size template only matches one resolution. We
/// keep one canonical template (captured at 2560x1440, the highest-detail icon we have),
/// pre-downscale it across a scale band at load, and take the best match over all scales.
/// This handles arbitrary resolutions with no per-resolution PNGs and no resolution
/// plumbing from callers.
pub struct LureMatcher {
    /// The canonical icon pre-scaled across the band (largest first). Built once at load.
    templates: Vec<Mat>,
    threshold: f64,
}

/// Multi-scale band for the lure template. The canonical icon is captured at 2560x1440;
/// at lower render resolutions the on-screen icon is smaller, so we DOWNSCALE the
/// template. Empirically (TM_CCOEFF_NORMED) the correlation peak across scale is narrow —
/// a 1080p frame peaks at ~0.85 near scale 0.75 but only ~0.70 at the naive resolution
/// ratio 0.706 — so we sweep in fine 0.05 steps and take the global max. Downscale-only
/// (cap just above 1.0): upscaling a ~34px template blurs it and depresses the match, so
/// resolutions above the canonical's 1440 are out of scope (see the README roadmap).
const LURE_SCALE_MIN: f64 = 0.50;
const LURE_SCALE_MAX: f64 = 1.05;
const LURE_SCALE_STEP: f64 = 0.05;

impl LureMatcher {
    /// Load the canonical lure-buff icon and pre-build the multi-scale template bank.
    pub fn load(icon_path: impl AsRef<Path>, threshold: f64) -> anyhow::Result<Self> {
        let path = icon_path.as_ref();
        let file = path.to_str().context("lure icon path is not valid UTF-8")?;
        let canonical = opencv::imgcodecs::imread_def(file)
            .with_context(|| format!("loading lure icon {}", path.display()))?;
        anyhow::ensure!(
            !canonical.empty(),
            "lure icon is empty or unreadable: {}",
            path.display()
        );
        let templates = Self::build_scales(&canonical)?;
        anyhow::ensure!(
            !templates.is_empty(),
            "no usable lure template scales from {}",
            path.display()
        );
        Ok(Self {
            templates,
            threshold,
        })
    }

    /// Pre-scale the canonical template across the band, largest first. INTER_AREA is the
    /// correct resampler for shrinking (it averages source pixels, preserving the small
    /// icon's edges); INTER_LINEAR covers the rare at-or-above-1.0 step.
    fn build_scales(canonical: &Mat) -> anyhow::Result<Vec<Mat>> {
        let (cw, ch) = (canonical.cols(), canonical.rows());
        let steps = ((LURE_SCALE_MAX - LURE_SCALE_MIN) / LURE_SCALE_STEP).round() as i32;
        let mut templates = Vec::new();
        for i in (0..=steps).rev() {
            let scale = LURE_SCALE_MIN + LURE_SCALE_STEP * f64::from(i);
            let nw = (f64::from(cw) * scale).round() as i32;
            let nh = (f64::from(ch) * scale).round() as i32;
            if nw < 4 || nh < 4 {
                continue; // too small to correlate meaningfully
            }
            if nw == cw && nh == ch {
                templates.push(canonical.try_clone()?); // scale ~1.0: no resample
                continue;
            }
            let interp = if scale < 1.0 {
                opencv::imgproc::INTER_AREA
            } else {
                opencv::imgproc::INTER_LINEAR
            };
            let mut scaled = Mat::default();
            opencv::imgproc::resize(canonical, &mut scaled, Size::new(nw, nh), 0.0, 0.0, interp)?;
            templates.push(scaled);
        }
        Ok(templates)
    }

    /// Best multi-scale template-match score (0–1) for the lure icon anywhere in `bgr`.
    /// Templates larger than `bgr` in either dimension are skipped (match_template
    /// requires template <= image); if every template is skipped the score is 0.0.
    pub fn score(&self, bgr: &Mat) -> anyhow::Result<f64> {
        let (rw, rh) = (bgr.cols(), bgr.rows());
        let mut best = 0.0_f64;
        for template in &self.templates {
            if template.cols() > rw || template.rows() > rh {
                continue;
            }
            let mut result = Mat::default();
            opencv::imgproc::match_template_def(
                bgr,
                template,
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
            best = best.max(max_val);
        }
        Ok(best)
    }

    /// True if the lure buff is found above the confidence threshold.
    pub fn present(&self, bgr: &Mat) -> anyhow::Result<bool> {
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

    fn canonical_lure_matcher() -> LureMatcher {
        let p = concat!(env!("CARGO_MANIFEST_DIR"), "/icons/lure_2560x1440.png");
        LureMatcher::load(p, 0.7).expect("load canonical lure template")
    }

    #[test]
    fn multiscale_lure_matches_native_1080p_icon() {
        use opencv::core::{Scalar, CV_8UC3};
        use opencv::imgcodecs;

        // The matcher is built from the 2560x1440 canonical (pre-downscaled across the
        // band). The fixture it must locate is the committed *native-1080p* render — so a
        // pass proves cross-resolution matching, which the old fixed-scale matcher failed.
        let matcher = canonical_lure_matcher();
        let icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/icons/lure_1920x1080.png");
        let icon = imgcodecs::imread_def(icon_path).expect("read 1080p icon fixture");
        assert!(!icon.empty(), "1080p icon fixture is empty");

        // Paste the icon onto a blank BGR canvas the size of a 1080p top-right quadrant.
        let mut frame =
            Mat::new_rows_cols_with_default(540, 960, CV_8UC3, Scalar::all(0.0)).unwrap();
        let dst = Rect::new(700, 40, icon.cols(), icon.rows());
        {
            let mut roi = frame.roi_mut(dst).expect("roi_mut into frame");
            icon.copy_to(&mut roi).expect("copy icon into frame region");
        } // drop the mutable borrow before scoring

        let score = matcher.score(&frame).expect("score runs");
        assert!(
            score >= 0.7,
            "canonical multi-scale matcher should find the native-1080p icon, got {score:.3}"
        );

        // A blank frame must not register the lure (no false positive).
        let blank = Mat::new_rows_cols_with_default(540, 960, CV_8UC3, Scalar::all(0.0)).unwrap();
        assert!(
            !matcher.present(&blank).expect("present runs on blank"),
            "a blank frame must not register the lure as present"
        );
    }
}
