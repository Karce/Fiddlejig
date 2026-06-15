//! Learned (ONNX) bobber detector, run in pure Rust via `tract`.
//!
//! A YOLO11-n detect head — exported with `nms=False` so the graph stays
//! `tract`-friendly — replaces the Haar cascade. It generalizes across scenery
//! (zones) where the cascade was brittle. The flow: letterbox a (optional) ROI crop
//! of the frame to the model's square input, run inference, decode the
//! `(1, 4+nc, anchors)` head, apply a confidence gate + IoU NMS in Rust, and map the
//! surviving box centers **back to full-frame coordinates** — so click points stay
//! in frame/portal space, exactly like the cascade's outputs.

use crate::detect::Detector;
use crate::frame::Frame;
use crate::state::Point;
use anyhow::Result;
use opencv::core::{Mat, Rect, Scalar, Size, BORDER_CONSTANT};
use opencv::prelude::*;
use std::path::Path;
use tract_onnx::prelude::*;

/// The optimized, runnable typed plan produced by `into_runnable()`.
type OnnxModel = TypedRunnableModel<TypedModel>;

/// One decoded detection, in letterboxed model-input pixel coordinates.
struct Det {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    score: f32,
}

pub struct NnDetector {
    model: OnnxModel,
    /// Square model input edge in px — must equal the export `imgsz`.
    input: i32,
    conf_threshold: f32,
    iou_threshold: f32,
    /// Optional normalized `(x, y, w, h)` crop region; `None` = whole frame.
    roi: Option<(f64, f64, f64, f64)>,
}

impl NnDetector {
    /// Load an exported YOLO ONNX and pin its input shape so `tract` can optimize.
    pub fn load(
        model_path: impl AsRef<Path>,
        input: i32,
        conf_threshold: f32,
        iou_threshold: f32,
        roi: Option<(f64, f64, f64, f64)>,
    ) -> Result<Self> {
        let path = model_path.as_ref();
        anyhow::ensure!(path.exists(), "ONNX model not found: {}", path.display());
        let n = input as usize;
        let model = tract_onnx::onnx()
            .model_for_path(path)?
            .with_input_fact(0, f32::fact([1, 3, n, n]).into())?
            .into_optimized()?
            .into_runnable()?;
        Ok(Self {
            model,
            input,
            conf_threshold,
            iou_threshold,
            roi,
        })
    }

    /// Pixel ROI rect (clamped to the frame) plus its origin, for back-mapping.
    fn roi_rect(&self, fw: i32, fh: i32) -> (i32, i32, i32, i32) {
        match self.roi {
            Some((rx, ry, rw, rh)) => {
                let x = ((rx * fw as f64).round() as i32).clamp(0, fw - 1);
                let y = ((ry * fh as f64).round() as i32).clamp(0, fh - 1);
                let w = ((rw * fw as f64).round() as i32).clamp(1, fw - x);
                let h = ((rh * fh as f64).round() as i32).clamp(1, fh - y);
                (x, y, w, h)
            }
            None => (0, 0, fw, fh),
        }
    }
}

impl Detector for NnDetector {
    fn detect(&mut self, frame: &Frame) -> Result<Vec<Point>> {
        let full = frame.to_mat()?;
        let (fw, fh) = (frame.width as i32, frame.height as i32);
        let (ox, oy, cw, ch) = self.roi_rect(fw, fh);
        let crop = Mat::roi(&full, Rect::new(ox, oy, cw, ch))?.try_clone()?;

        // letterbox the crop into a square `input`×`input` canvas (gray 114 pad),
        // matching Ultralytics so the bobber's apparent scale is the same as in
        // training.
        let n = self.input;
        let r = (n as f64 / cw as f64).min(n as f64 / ch as f64);
        let new_w = (cw as f64 * r).round() as i32;
        let new_h = (ch as f64 * r).round() as i32;
        let mut resized = Mat::default();
        opencv::imgproc::resize(
            &crop,
            &mut resized,
            Size::new(new_w, new_h),
            0.0,
            0.0,
            opencv::imgproc::INTER_LINEAR,
        )?;
        let (left, top) = ((n - new_w) / 2, (n - new_h) / 2);
        let (right, bottom) = (n - new_w - left, n - new_h - top);
        let mut padded = Mat::default();
        opencv::core::copy_make_border(
            &resized,
            &mut padded,
            top,
            bottom,
            left,
            right,
            BORDER_CONSTANT,
            Scalar::new(114.0, 114.0, 114.0, 0.0),
        )?;
        let mut rgb = Mat::default();
        opencv::imgproc::cvt_color_def(&padded, &mut rgb, opencv::imgproc::COLOR_BGR2RGB)?;

        // HWC u8 → NCHW f32 / 255
        let bytes = rgb.data_bytes()?;
        let nn = n as usize;
        let tensor: Tensor =
            tract_ndarray::Array4::<f32>::from_shape_fn((1, 3, nn, nn), |(_, c, y, x)| {
                bytes[(y * nn + x) * 3 + c] as f32 / 255.0
            })
            .into();

        let result = self.model.run(tvec!(tensor.into()))?;
        let view = result[0]
            .to_array_view::<f32>()?
            .into_dimensionality::<tract_ndarray::Ix3>()?;
        let shape = view.shape().to_vec();
        anyhow::ensure!(shape[0] == 1, "unexpected output batch {:?}", shape);

        // head is (1, 4+nc, anchors); some exports transpose to (1, anchors, 4+nc).
        // the attribute axis (4 box coords + nc class scores) is the smaller one.
        let transposed = shape[1] > shape[2];
        let (n_attr, n_anchor) = if transposed {
            (shape[2], shape[1])
        } else {
            (shape[1], shape[2])
        };
        anyhow::ensure!(n_attr >= 5, "expected >=5 output attributes, got {n_attr}");
        let at = |attr: usize, anchor: usize| -> f32 {
            if transposed {
                view[[0, anchor, attr]]
            } else {
                view[[0, attr, anchor]]
            }
        };

        let mut dets: Vec<Det> = Vec::new();
        for a in 0..n_anchor {
            // single class → attr 4 is the score; multi-class → max over class scores
            let mut score = at(4, a);
            for k in 5..n_attr {
                score = score.max(at(k, a));
            }
            if score < self.conf_threshold {
                continue;
            }
            let (cx, cy, w, h) = (at(0, a), at(1, a), at(2, a), at(3, a));
            dets.push(Det {
                x1: cx - w / 2.0,
                y1: cy - h / 2.0,
                x2: cx + w / 2.0,
                y2: cy + h / 2.0,
                score,
            });
        }

        // un-letterbox + un-crop each surviving center back to full-frame coords
        Ok(nms(dets, self.iou_threshold)
            .iter()
            .map(|d| {
                let cx_in = (d.x1 + d.x2) / 2.0;
                let cy_in = (d.y1 + d.y2) / 2.0;
                Point {
                    x: ox as f64 + ((cx_in - left as f32) / r as f32) as f64,
                    y: oy as f64 + ((cy_in - top as f32) / r as f32) as f64,
                }
            })
            .collect())
    }
}

/// Greedy IoU non-max suppression. Consumes the candidate list (sorted by score).
fn nms(mut dets: Vec<Det>, iou_thr: f32) -> Vec<Det> {
    dets.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut keep: Vec<Det> = Vec::new();
    'next: for d in dets {
        for k in &keep {
            if iou(&d, k) > iou_thr {
                continue 'next;
            }
        }
        keep.push(d);
    }
    keep
}

fn iou(a: &Det, b: &Det) -> f32 {
    let w = (a.x2.min(b.x2) - a.x1.max(b.x1)).max(0.0);
    let h = (a.y2.min(b.y2) - a.y1.max(b.y1)).max(0.0);
    let inter = w * h;
    let area_a = (a.x2 - a.x1).max(0.0) * (a.y2 - a.y1).max(0.0);
    let area_b = (b.x2 - b.x1).max(0.0) * (b.y2 - b.y1).max(0.0);
    let union = area_a + area_b - inter;
    if union <= 0.0 {
        0.0
    } else {
        inter / union
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model_path() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("models/bobber.onnx")
    }

    /// Smoke test: the committed ONNX loads in tract and the full
    /// preprocess→infer→decode→NMS path runs; a black frame yields no detections.
    /// Guards against tract op-compat regressions in the exported model.
    #[test]
    fn nn_loads_and_runs_on_blank_frame() {
        let p = model_path();
        if !p.exists() {
            eprintln!("model {} not present yet; skipping", p.display());
            return;
        }
        let mut det = NnDetector::load(&p, 960, 0.25, 0.45, None).expect("load onnx");
        let frame = Frame::new(vec![0u8; 960 * 960 * 3], 960, 960);
        let points = det.detect(&frame).expect("detection runs");
        assert!(points.is_empty(), "a blank frame should yield no bobbers");
    }

    /// Read an image file into a `Frame` (BGR), for the corpus tests.
    #[cfg(test)]
    fn load_frame(path: &std::path::Path) -> Option<Frame> {
        let mat = opencv::imgcodecs::imread_def(path.to_str()?).ok()?;
        if mat.empty() {
            return None;
        }
        let (w, h) = (mat.cols() as u32, mat.rows() as u32);
        Some(Frame::new(mat.data_bytes().ok()?.to_vec(), w, h))
    }

    /// Recall over the local (gitignored) positive corpus: the NN should find a
    /// bobber in most splash frames. Prints a per-zone breakdown.
    #[test]
    #[ignore = "needs the local, gitignored training corpus + a trained model"]
    fn nn_recall_on_local_corpus() {
        let p = model_path();
        assert!(
            p.exists(),
            "train + export the model first: {}",
            p.display()
        );
        let mut det = NnDetector::load(&p, 960, 0.25, 0.45, None).expect("load onnx");
        let root =
            std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../training/positive"));
        if !root.exists() {
            eprintln!("corpus not present; skipping");
            return;
        }
        let (mut images, mut with_hit) = (0u32, 0u32);
        for zone in std::fs::read_dir(root).unwrap().flatten() {
            if !zone.path().is_dir() {
                continue;
            }
            let (mut zi, mut zh) = (0u32, 0u32);
            for entry in std::fs::read_dir(zone.path()).unwrap().flatten() {
                let Some(frame) = load_frame(&entry.path()) else {
                    continue;
                };
                let hit = !det.detect(&frame).unwrap().is_empty();
                zi += 1;
                zh += hit as u32;
            }
            eprintln!(
                "  {:<14} {zh}/{zi}",
                zone.file_name().to_string_lossy().to_string()
            );
            images += zi;
            with_hit += zh;
        }
        eprintln!("nn recall: {with_hit}/{images}");
        assert!(with_hit * 100 >= images * 70, "recall below 70%");
    }

    /// Head-to-head: cascade vs NN over the whole local corpus — recall (positives),
    /// false-positive rate (negatives), and average inference time per image. This
    /// is the evidence for whether to flip the default backend to `Nn` (Phase 4).
    #[test]
    #[ignore = "needs the local corpus + trained model; prints a comparison"]
    fn compare_cascade_vs_nn() {
        use crate::detect::CascadeDetector;
        use std::time::Instant;

        let p = model_path();
        assert!(
            p.exists(),
            "train + export the model first: {}",
            p.display()
        );
        let training = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../training"));
        if !training.join("positive").exists() {
            eprintln!("corpus not present; skipping");
            return;
        }

        let cascade_model = concat!(env!("CARGO_MANIFEST_DIR"), "/models/bobber_z4_10v3.xml");
        let mut cascade = CascadeDetector::load(cascade_model, 2).expect("load cascade");
        let mut nn = NnDetector::load(&p, 960, 0.25, 0.45, None).expect("load onnx");

        // (images, images-with-a-hit, total inference ms) over <root>/<zone>/*
        fn sweep(det: &mut dyn Detector, root: &std::path::Path) -> (u32, u32, f64) {
            let (mut imgs, mut hits, mut ms) = (0u32, 0u32, 0.0f64);
            let Ok(zones) = std::fs::read_dir(root) else {
                return (0, 0, 0.0);
            };
            for zone in zones.flatten() {
                if !zone.path().is_dir() {
                    continue;
                }
                for entry in std::fs::read_dir(zone.path()).unwrap().flatten() {
                    let Some(frame) = load_frame(&entry.path()) else {
                        continue;
                    };
                    let t = Instant::now();
                    let n = det.detect(&frame).unwrap().len();
                    ms += t.elapsed().as_secs_f64() * 1000.0;
                    imgs += 1;
                    hits += (n > 0) as u32;
                }
            }
            (imgs, hits, ms)
        }

        let report = |name: &str, det: &mut dyn Detector| {
            let (pos_n, pos_hit, pos_ms) = sweep(det, &training.join("positive"));
            let (neg_n, neg_hit, neg_ms) = sweep(det, &training.join("negative"));
            eprintln!(
                "{name:>8}: recall {pos_hit}/{pos_n} ({:.1}%)  false-pos {neg_hit}/{neg_n} ({:.1}%)  avg {:.1} ms/img",
                100.0 * pos_hit as f64 / pos_n.max(1) as f64,
                100.0 * neg_hit as f64 / neg_n.max(1) as f64,
                (pos_ms + neg_ms) / (pos_n + neg_n).max(1) as f64,
            );
        };
        report("cascade", &mut cascade);
        report("nn", &mut nn);
    }
}
