use rand::RngCore;

use crate::*;

#[derive(Debug)]
pub(crate) struct BirdIndividual {
  pub(crate) fitness: f64,
  pub(crate) chromosome: ga::Chromosome,
}

impl BirdIndividual {
  pub(crate) fn into_bird(self, rng: &mut impl RngCore) -> Bird {
    Bird::from_chromosome(self.chromosome, rng)
  }
}

impl ga::Individual for BirdIndividual {
  fn fitness(&self) -> f64 {
    self.fitness
  }
}

impl AsRef<ga::Chromosome> for BirdIndividual {
  fn as_ref(&self) -> &ga::Chromosome {
    &self.chromosome
  }
}

impl From<ga::Chromosome> for BirdIndividual {
  fn from(chromosome: ga::Chromosome) -> Self {
    Self {
      fitness: 0.0,
      chromosome
    }
  }
}

impl From<Bird> for BirdIndividual {
  fn from(bird: Bird) -> Self {
    Self {
      fitness: bird.fit_distance,
      chromosome: bird.brain.chromosome(),
    }
  }
}