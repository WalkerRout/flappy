
use nalgebra as na;

use crate::*;

pub(crate) struct Rectangle {
  pub(crate) position: na::Point2<f64>, // bot left
  pub(crate) width: f64,
  pub(crate) height: f64,
}

impl AABB for Rectangle {
  fn top(&self) -> f64 {
    self.position.y + self.height
  }

  fn right(&self) -> f64 {
    self.position.x + self.width
  }

  fn bot(&self) -> f64 {
    self.position.y
  }

  fn left(&self) -> f64 {
    self.position.x
  }
}