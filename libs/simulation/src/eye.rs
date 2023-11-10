use nalgebra as na;

use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct Eye;

impl Eye {
  pub(crate) fn new() -> Self {
    Self
  }

  pub(crate) fn step_vision(&self, dy: f64, position: na::Point2<f64>, target: na::Point2<f64>) -> Vec<f64> {
    // ypos of bird (dist from bot), y dist from top, y dist to bottom pipe, y dist to top pipe, x dist to target, yvel of bird
    vec![
      position.y,
      (target.y - PIPE_OFFSET_Y) - position.y,
      (target.y + PIPE_OFFSET_Y) - position.y,
      target.x - position.x,
      dy,
    ]
  }
}