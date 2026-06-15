//! A captured frame: owned BGR pixels plus dimensions, decoupled from GStreamer
//! and OpenCV types so it can cross thread boundaries cheaply.

use crate::state::Point;
use opencv::core::{Mat, Scalar, CV_8UC3};
use opencv::prelude::*;

#[derive(Clone)]
pub struct Frame {
    /// Tightly packed BGR pixels, row-major, length `width * height * 3`.
    pub bgr: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Frame {
    pub fn new(bgr: Vec<u8>, width: u32, height: u32) -> Self {
        Self { bgr, width, height }
    }

    /// Frame center in frame coordinates (bobbers nearest it are preferred).
    pub fn center(&self) -> Point {
        Point {
            x: self.width as f64 / 2.0,
            y: self.height as f64 / 2.0,
        }
    }

    /// Copy the BGR pixels into an owned, contiguous `CV_8UC3` `Mat` for OpenCV.
    pub fn to_mat(&self) -> opencv::Result<Mat> {
        let mut mat = Mat::new_rows_cols_with_default(
            self.height as i32,
            self.width as i32,
            CV_8UC3,
            Scalar::all(0.0),
        )?;
        mat.data_bytes_mut()?.copy_from_slice(&self.bgr);
        Ok(mat)
    }
}
