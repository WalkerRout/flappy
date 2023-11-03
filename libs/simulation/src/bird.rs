use rand::{Rng, RngCore};
use nalgebra as na;

use crate::*;

pub const BIRD_X: f64 = 0.15;

pub const BIRD_OFFSET_X: f64 = 0.012;
pub const BIRD_OFFSET_Y: f64 = 0.012;

#[derive(Debug, Clone)]
pub struct Bird {
  pub position: na::Point2<f64>,
  pub dy: f64,
  pub fit_distance: f64,
  pub passes: usize,
  jump_timer: usize,
  pub(crate) eye: Eye,
  pub(crate) brain: Brain,
}

impl Bird {
  pub fn random(rng: &mut impl RngCore) -> Self {
    Self::new(Brain::random(rng), rng)
  }

  pub fn new(brain: Brain, rng: &mut impl RngCore) -> Self {
    Self { 
      position: na::Point2::new(BIRD_X, rng.gen()),
      dy: 0.005,
      fit_distance: 0.01,
      passes: 0,
      jump_timer: 0,
      eye: Eye::new(),
      brain,
    }
  }

  pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut impl RngCore) -> Self {
    let brain = Brain::from_chromosome(chromosome);
    Self::new(brain, rng)
  }

  pub(crate) fn collision(&self, closest_pipe: Option<&Pipe>) -> bool {
    self.collision_screen() || self.collision_pipe(closest_pipe)
  }

  pub(crate) fn decision(&mut self, closest_pipe: na::Point2<f64>) {
    let vision = self.eye.step_vision(self.dy, self.position, closest_pipe);
    let response = self.brain.nn.propagate(vision);

    // jump
    if response[0] > 0.5 && self.jump_timer > 19 {
      self.dy += 0.019;
      self.jump_timer = 0;
    }

    self.jump_timer += 1;
  }

  pub(crate) fn movement(&mut self) { 
    // gravity
    self.dy += -0.0006;
    self.dy = self.dy.max(-0.014).min(0.015);

    // add
    self.position.y += self.dy;
    self.fit_distance = 20.0*self.passes as f64 + 0.1;
  }

  fn collision_screen(&self) -> bool {
    let y = self.position.y;
    y < BIRD_OFFSET_Y || y > 1.0 - BIRD_OFFSET_Y
  }

  fn collision_pipe(&self, closest_pipe: Option<&Pipe>) -> bool {    
    if let Some(pipe) = closest_pipe {
      self.collision_aabb(pipe.top_rectangle()) || self.collision_aabb(pipe.bot_rectangle())
    } else {
      false
    }
  }

  fn collision_aabb(&self, other: impl AABB) -> bool {
    self.intersect(&other)
  }
}

impl Bird {
  pub fn dy(&self) -> f64 {
    self.dy
  }

  pub fn position(&self) -> na::Point2<f64> {
    self.position
  }

  pub fn fit_distance(&self) -> f64 {
    self.fit_distance
  }
}

impl AABB for Bird {
  fn top(&self) -> f64 {
    self.position.y + BIRD_OFFSET_Y
  }

  fn right(&self) -> f64 {
    self.position.x + BIRD_OFFSET_X
  }

  fn bot(&self) -> f64 {
    self.position.y - BIRD_OFFSET_Y
  }

  fn left(&self) -> f64 {
    self.position.x - BIRD_OFFSET_X
  }
}