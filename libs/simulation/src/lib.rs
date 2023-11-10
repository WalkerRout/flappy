use rand::RngCore;
use nalgebra as na;

pub use lib_neural_network as nn;
pub use lib_genetic_algorithm as ga;

mod eye;
mod bird;
mod pipe;
mod brain;
mod world;
mod bird_individual;

pub use self::{
  eye::*,
  bird::*,
  pipe::*,
  brain::*,
  world::*,
  bird_individual::*,
};

pub trait AABB {
  fn top(&self) -> f64;
  fn right(&self) -> f64;
  fn bot(&self) -> f64;
  fn left(&self) -> f64;

  fn intersect<O: AABB>(&self, other: &O) -> bool {
    self.left() < other.right()
      && self.right() > other.left()
      && self.bot() < other.top()
      && self.top() > other.bot()
  }
}

type GeneticAlgorithm = ga::GeneticAlgorithm<
  ga::RouletteWheelSelection, 
  ga::UniformCrossover, 
  ga::GaussianMutation,
>;

pub struct Simulation {
  world: World,
  tick_count: usize,
  generations: usize,
  genetic_alg: GeneticAlgorithm,
}

impl Simulation {
  pub fn random(rng: &mut impl RngCore) -> Self {
    let genetic_alg = GeneticAlgorithm::new(
      ga::RouletteWheelSelection::default(),
      ga::UniformCrossover::default(),
      ga::GaussianMutation::new(0.017, 0.3),
    );

    Self {
      world: World::random(rng),
      tick_count: 0,
      generations: 0,
      genetic_alg,
    }
  }

  pub fn step(&mut self, rng: &mut impl RngCore) -> bool {
    let ticks = self.tick_count;
    self.tick_count += 1;

    self.prepare_world(ticks, rng);
    self.step_world();

    if self.world.alive_birds.is_empty() {
      self.evolve(rng);

      self.tick_count = 0;
      self.generations += 1;

      true
    } else {
      false
    }
  }

  pub fn train(&mut self, rng: &mut impl RngCore) {
    let new_tick_count = self.tick_count + 10000;
    loop {
      if self.step(rng) || self.tick_count >= new_tick_count {
        return;
      }
    }
  }

  fn prepare_world(&mut self, ticks: usize, rng: &mut impl RngCore) {
    // check collisions
    self.world.collision();

    // spawn new pipes
    if ticks % PIPE_TICK_GENERATION == 0 {
      self.world.push_pipe(rng);
    }
  }

  fn step_world(&mut self) {
    // make movements
    self.world.decision();
    self.world.movement();
  }

  fn evolve(&mut self, rng: &mut impl RngCore) {
    // prepare population
    let curr_pop: Vec<BirdIndividual> = self.world.birds_as_individuals();

    // evolve population
    let (evo_pop, _) = self.genetic_alg.evolve(rng, &curr_pop);
    
    // bring back population
    self.world.alive_birds = self.world.individuals_as_birds(evo_pop, rng);

    // reset environment
    self.reset();
  }

  fn reset(&mut self) {
    self.world.pipes.clear();
    self.world.dead_birds.clear();
  }
}

impl Simulation {
  pub fn world(&self) -> &World {
    &self.world
  }

  pub fn tick_count(&self) -> usize {
    self.tick_count
  }
}