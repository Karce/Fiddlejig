//! Runtime configuration. Defaults live in code; an optional `config.toml` can
//! override any field (via `serde`), and CLI flags override that.

use crate::state::Point;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Which detection backend the bot runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DetectorBackend {
    /// OpenCV Haar cascade (fast, scenery-brittle).
    Cascade,
    /// Learned YOLO ONNX via tract (robust across zones).
    Nn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Startup grace period before the first cast (seconds).
    pub initializing_secs: u64,
    /// Recast if no bobber is clicked within this long (seconds).
    pub fishing_duration_secs: u64,
    /// How long applying a lure takes before we can cast (seconds).
    pub lure_cast_secs: u64,
    /// Quiet period after casting before the bot watches for a bite — lets the
    /// cast animation / idle bobber settle so it won't false-trigger or reel early.
    pub settle_secs: u64,
    /// Pause after reeling in a fish before recasting — lets the catch/loot resolve
    /// so a lure re-apply keypress isn't swallowed by the post-catch moment (ms).
    pub post_catch_ms: u64,
    /// Action-bar keys pressed to cast. One key is recommended — pressing Fishing
    /// twice back-to-back cancels the first cast.
    pub cast_keys: Vec<char>,
    /// Action-bar key that applies the lure.
    pub lure_key: char,
    /// Skip lure detection/application entirely (e.g. no template for this resolution).
    pub skip_lure: bool,
    /// Cascade model to load, relative to the crate root.
    pub model: String,
    /// Where to park the cursor after casting so it doesn't cover the bobber
    /// (frame coordinates).
    pub mouse_park: (f64, f64),
    /// Lure-buff template image, relative to the crate root (None until captured).
    pub lure_icon: Option<String>,
    /// Capture framerate cap (frames/sec), enforced by GStreamer `videorate`. The
    /// splash lasts ~1–2s, so ~10fps catches it with margin while slashing the cost
    /// of the per-frame BGR readback and detection vs. the stream's native ~60fps.
    pub target_fps: u32,

    // --- detector backend ---
    /// Which detector to run: the Haar cascade or the learned ONNX model.
    pub backend: DetectorBackend,
    /// ONNX model path (relative to the crate root), used when `backend = nn`.
    pub nn_model: String,
    /// Square model input edge in px — must equal the export `imgsz`.
    pub nn_input_size: i32,
    /// Confidence threshold (0–1) for NN detections.
    pub nn_conf_threshold: f32,
    /// NMS IoU threshold (0–1) for overlapping NN detections.
    pub nn_iou_threshold: f32,
    /// Optional ROI to crop before detection, normalized `(x, y, w, h)` in [0,1];
    /// `None` = whole frame. Click coords are always mapped back to the full frame.
    pub roi: Option<(f64, f64, f64, f64)>,

    // --- detection tuning ---
    /// Cascade `min_neighbors` — higher rejects more weak detections (false positives).
    pub min_neighbors: i32,
    /// A bobber must stay detected (within `stability_radius` px) for this long
    /// before the bot reels — filters transient false positives (milliseconds).
    pub stability_ms: u64,
    /// Tolerate detection drop-outs up to this long without losing the streak —
    /// real bobber detection flickers frame-to-frame (milliseconds).
    pub flicker_ms: u64,
    /// How close (px) successive detections must be to count as the same bobber.
    pub stability_radius: f64,
    /// Min template-match confidence (0–1) to consider the lure buff present.
    pub lure_threshold: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            initializing_secs: 10,
            fishing_duration_secs: 30,
            lure_cast_secs: 10,
            settle_secs: 3,
            post_catch_ms: 1500,
            cast_keys: vec!['1'],
            lure_key: '3',
            skip_lure: false,
            model: "models/bobber_z4_10v3.xml".to_string(),
            mouse_park: (100.0, 500.0),
            lure_icon: Some("icons/lure_2560x1440.png".to_string()),
            target_fps: 10,
            // default to the cascade until the NN is validated to beat it (Phase 4);
            // nn_input_size must match the export imgsz (trained at 960).
            backend: DetectorBackend::Cascade,
            nn_model: "models/bobber.onnx".to_string(),
            nn_input_size: 960,
            nn_conf_threshold: 0.25,
            nn_iou_threshold: 0.45,
            roi: None,
            min_neighbors: 2,
            stability_ms: 350,
            flicker_ms: 250,
            stability_radius: 40.0,
            lure_threshold: 0.7,
        }
    }
}

impl Config {
    pub fn initializing(&self) -> Duration {
        Duration::from_secs(self.initializing_secs)
    }

    pub fn fishing_duration(&self) -> Duration {
        Duration::from_secs(self.fishing_duration_secs)
    }

    pub fn lure_cast(&self) -> Duration {
        Duration::from_secs(self.lure_cast_secs)
    }

    pub fn settle(&self) -> Duration {
        Duration::from_secs(self.settle_secs)
    }

    pub fn post_catch(&self) -> Duration {
        Duration::from_millis(self.post_catch_ms)
    }

    pub fn stability(&self) -> Duration {
        Duration::from_millis(self.stability_ms)
    }

    pub fn flicker_grace(&self) -> Duration {
        Duration::from_millis(self.flicker_ms)
    }

    pub fn mouse_park(&self) -> Point {
        Point {
            x: self.mouse_park.0,
            y: self.mouse_park.1,
        }
    }
}
