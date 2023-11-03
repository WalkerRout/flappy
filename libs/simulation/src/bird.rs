use rand::{Rng, RngCore};
use nalgebra as na;

use crate::*;

pub const BIRD_X: f64 = 0.15;
pub const BIRD_RADIUS: f64 = 0.017;

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

  pub(crate) fn movement(&mut self, closest_pipe: na::Point2<f64>, _ticks: usize) { 
    let vision = self.eye.step_vision(self.dy, self.position, closest_pipe);
    let response = self.brain.nn.propagate(vision);

    // jump
    if response[0] > 0.5 && self.jump_timer > 19 {
      self.dy += 0.018;
      self.jump_timer = 0;
    }

    self.jump_timer += 1;

    // gravity
    self.dy += -0.0006;
    self.dy = self.dy.max(-0.01).min(0.015);

    // add
    self.position.y += self.dy;
    self.fit_distance += 20.0*self.passes as f64;
  }

  pub(crate) fn collision(&self, closest_pipe: Option<&Pipe>) -> bool {
    self.collision_screen() || self.collision_pipes(closest_pipe)
  }

  pub(crate) fn collision_screen(&self) -> bool {
    let y = self.position.y;
    y < BIRD_RADIUS || y > 1.0 - BIRD_RADIUS
  }

  pub(crate) fn collision_pipes(&self, closest_pipe: Option<&Pipe>) -> bool {    
    if let Some(pipe) = closest_pipe {
      self.collision_pipe_top(pipe) || self.collision_pipe_bot(pipe)
    } else {
      false
    }
  }

  pub(crate) fn collision_pipe_top(&self, closest_pipe: &Pipe) -> bool {
    let (x, y) = (self.position.x, self.position.y);
    let (px, py) = (closest_pipe.position.x, closest_pipe.position.y);
      
    y + BIRD_RADIUS > py + PIPE_OFFSET_Y
      && x + BIRD_RADIUS > px - PIPE_OFFSET_X
      && x - BIRD_RADIUS < px + PIPE_OFFSET_X
  }

  pub(crate) fn collision_pipe_bot(&self, closest_pipe: &Pipe) -> bool {
    let (x, y) = (self.position.x, self.position.y);
    let (px, py) = (closest_pipe.position.x, closest_pipe.position.y);
      
    y - BIRD_RADIUS < py - PIPE_OFFSET_Y
      && x + BIRD_RADIUS > px - PIPE_OFFSET_X
      && x - BIRD_RADIUS < px + PIPE_OFFSET_X
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