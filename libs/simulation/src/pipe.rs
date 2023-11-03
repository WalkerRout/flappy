use rand::{Rng, RngCore};
use nalgebra as na;

pub const PIPE_DX: f64 = 0.0045;
pub const PIPE_OFFSET_X: f64 = 0.044; // pipe/gap width = 2*PIPE_OFFSET_X
pub const PIPE_OFFSET_Y: f64 = 0.13; // pipe gap height = 2*PIPE_OFFSET_Y
pub const PIPE_TICK_GENERATION: usize = 170;

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
    let y = clamp(rng.gen(), PIPE_OFFSET_Y, 1.0 - PIPE_OFFSET_Y);

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

  pub(crate) fn top(&self) -> (f64, f64, f64, f64) {
    let top = 1.0;
    let top_safe = self.position.y + PIPE_OFFSET_Y;
    let height = top - top_safe;

    (self.position.x, top_safe + height / 2.0, PIPE_OFFSET_X, height / 2.0)
  }

  pub(crate) fn bot(&self) -> (f64, f64, f64, f64) {
    let bot = 0.0;
    let bot_safe = self.position.y - PIPE_OFFSET_Y;
    let height = bot_safe - bot;

    (self.position.x, bot_safe - height / 2.0, PIPE_OFFSET_X, height / 2.0)
  }
}

impl Pipe {
  pub fn position(&self) -> na::Point2<f64> {
    self.position
  }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
  x.max(min).min(max)
}