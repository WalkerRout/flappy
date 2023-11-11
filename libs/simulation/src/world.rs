use crate::*;

use std::mem;
use std::iter;
use std::collections::VecDeque;

const POPULATION_COUNT: usize = 70;

#[derive(Debug)]
pub struct World {
  pub alive_birds: Vec<Bird>,
  pub dead_birds: Vec<Bird>,
  pub pipes: VecDeque<Pipe>,
}

impl World {
  pub(crate) fn random(rng: &mut impl RngCore) -> Self {
    let alive_birds = iter::from_fn(|| Some(Bird::random(rng)))
      .take(POPULATION_COUNT)
      .collect();

    Self { 
      alive_birds,
      dead_birds: Vec::with_capacity(POPULATION_COUNT),
      pipes: VecDeque::new(),
    }
  }

  pub(crate) fn collision(&mut self) {
    self.bird_collision(); // walls, pipes
    self.pipe_collision(); // end of screen -x
  }

  pub(crate) fn decision(&mut self) {
    let closest_pos = self.next_pipe()
      .map(|p| p.position)
      .unwrap_or(na::Point2::new(1.0, 0.5));

    self.bird_decision(closest_pos);
  }

  pub(crate) fn movement(&mut self) {
    self.bird_movement(); // +/- y
    self.pipe_movement(); // -x
  }

  pub(crate) fn birds_as_individuals(&mut self) -> Vec<BirdIndividual> {
    // steal allocations
    let alive = mem::take(&mut self.alive_birds);
    let dead  = mem::take(&mut self.dead_birds);

    alive
      .into_iter()
      .chain(dead.into_iter())
      .map(Into::into)
      .collect()
  }

  pub(crate) fn individuals_as_birds(&self, population: Vec<BirdIndividual>, rng: &mut impl RngCore) -> Vec<Bird> {
    population
      .into_iter()
      .map(|bi| bi.into_bird(rng))
      .collect()
  }

  fn bird_collision(&mut self) {
    let closest_pipe = self.next_pipe();
    let dead_birds = self.alive_birds
      .extract_if(|bird| bird.collision(closest_pipe.as_ref())); // #![feature(extract_if)]

    self.dead_birds.extend(dead_birds);
  }

  fn bird_decision(&mut self, closest_pos: na::Point2<f64>) {
    self.alive_birds
      .iter_mut()
      .for_each(|bird| bird.decision(closest_pos));
  }

  fn bird_movement(&mut self) {
    self.alive_birds
      .iter_mut()
      .for_each(|bird| bird.movement());
  }

  fn pipe_collision(&mut self) {
    self.pipes
      .iter()
      .for_each(|pipe| {
        if pipe.collision() {
          self.alive_birds
            .iter_mut()
            .for_each(|bird| {
              bird.passes += 1;
            });
        }
      });

    self.pipes
      .retain(|pipe| pipe.position.x > -PIPE_OFFSET_X);
  }

  fn pipe_movement(&mut self) {
    self.pipes
      .iter_mut()
      .for_each(|pipe| pipe.movement());
  }

  pub(crate) fn push_pipe(&mut self, rng: &mut impl RngCore) {
    self.pipes.push_back(Pipe::random(rng));
  }

  pub(crate) fn next_pipe(&self) -> Option<Pipe> {
    self.pipes
      .iter()
      .filter(|pipe| pipe.position.x > BIRD_X - PIPE_OFFSET_X) // filter pipes past bird
      .min_by(|a, b| {
        a.position.x.partial_cmp(&b.position.x).unwrap_or(std::cmp::Ordering::Equal)
      })
      .cloned()
  }
}

impl World {
  pub fn birds<'a>(&'a self) -> impl Iterator<Item=&'a Bird> {
    self.alive_birds.iter()
  }

  pub fn pipes<'a>(&'a self) -> impl Iterator<Item=&'a Pipe> {
    self.pipes.iter()
  }
}