use crate::*;

use std::iter;
use std::collections::VecDeque;

const POPULATION_COUNT: usize = 125;

#[derive(Debug)]
pub struct World {
  pub alive_birds: Vec<Bird>,
  pub dead_birds: Vec<Bird>,
  pub pipes: VecDeque<Pipe>,
  // ahead_pipes: Vec<Pipe>, // spawn new pipes here,
  // behind_pipes: Vec<Pipe>, // if pipe.x + xoffset < bird.x - radbird, move here
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

  pub(crate) fn movement(&mut self, ticks: usize) {
    let closest_pos = self.next_pipe()
      .map(|p| p.position)
      .unwrap_or(na::Point2::new(1.0, 0.5));

    self.bird_movement(closest_pos, ticks); // +/- y
    self.pipe_movement(); // -x
  }

  pub(crate) fn birds_as_individuals(&self) -> Vec<BirdIndividual> {
    self.alive_birds
      .iter()
      .chain(self.dead_birds.iter())
      .map(Into::into)
      .collect()
  }

  pub(crate) fn individuals_as_birds(&mut self, population: Vec<BirdIndividual>, rng: &mut impl RngCore) -> Vec<Bird> {
    population
      .into_iter()
      .map(|bi| bi.into_bird(rng))
      .collect()
  }

  fn bird_collision(&mut self) {
    let mut birds_to_move_to_dead: Vec<Bird> = Vec::new();
    let closest_pipe = self.next_pipe();

    self.alive_birds
      .retain(|bird| {
        let collision = bird.collision(closest_pipe.as_ref());

        if collision {
          let mut new_bird = bird.clone();
          new_bird.fit_distance /= 2.0;
          birds_to_move_to_dead.push(new_bird);
        }

        !collision
      });

    // Extend the dead_birds vector with collided birds
    self.dead_birds.extend(birds_to_move_to_dead);
  }

  fn bird_movement(&mut self, closest_pos: na::Point2<f64>, ticks: usize) {
    self.alive_birds
      .iter_mut()
      .for_each(|bird| bird.movement(closest_pos, ticks));
  }

  fn pipe_collision(&mut self) {
    self.pipes
      .iter()
      .for_each(|pipe| {
        if pipe.position.x <= -PIPE_OFFSET_X {
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
      .filter(|pipe| pipe.position.x > BIRD_X) // Filter pipes with x > 0.15
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