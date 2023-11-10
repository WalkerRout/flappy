use rand::{Rng, RngCore};
use nalgebra as na;

use crate::*;

pub const PIPE_DX: f64 = 0.0045;
pub const PIPE_OFFSET_X: f64 = 0.044; // pipe/gap width = 2*PIPE_OFFSET_X
pub const PIPE_OFFSET_Y: f64 = 0.13; // pipe gap height = 2*PIPE_OFFSET_Y
pub const PIPE_TICK_GENERATION: usize = 120;

#[derive(Debug, Clone)]
pub struct Pipe {
  // |     |
  // |     |
  // L_____J
  //    x      <- represents position in between pipes
  // f^^^^^7
  // |     |
  // |     |
  //
  pub(crate) position: na::Point2<f64>,
}

impl Pipe {
  pub(crate) fn random(rng: &mut impl RngCore) -> Self {
    let x = 1.0 + PIPE_OFFSET_X;
    let y = rng.gen::<f64>().max(PIPE_OFFSET_Y).min(1.0 - PIPE_OFFSET_Y);

    Self {
      position: na::Point2::new(x, y),
    }
  }

  pub(crate) fn collision(&self) -> bool {
    self.position.x <= -PIPE_OFFSET_X
  }

  pub(crate) fn movement(&mut self) {
    self.position.x += -PIPE_DX;
  }

  pub(crate) fn top_rectangle(&self) -> Rectangle {
    let position = na::Point2::new(self.position.x - PIPE_OFFSET_X, self.position.y + PIPE_OFFSET_Y);
    let width = 2.0 * PIPE_OFFSET_X;
    let height = 1.0 - position.y;

    Rectangle { 
      position,
      width,
      height,
    }
  }

  pub(crate) fn bot_rectangle(&self) -> Rectangle {
    let position = na::Point2::new(self.position.x - PIPE_OFFSET_X, 0.0);
    let width = 2.0 * PIPE_OFFSET_X;
    let height = self.position.y - PIPE_OFFSET_Y;

    Rectangle { 
      position,
      width,
      height,
    }
  }
}

impl Pipe {
  pub fn position(&self) -> na::Point2<f64> {
    self.position
  }
}

pub struct Rectangle {
  position: na::Point2<f64>, // bot left
  width: f64,
  height: f64,
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