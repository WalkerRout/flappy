#![feature(impl_trait_in_assoc_type)]

use rand::Rng;
use rand::RngCore;
use rand::seq::SliceRandom;

use std::ops::Index;

pub trait Individual: AsRef<Chromosome> + From<Chromosome> {
  fn fitness(&self) -> f64;
}

pub trait SelectionMethod {
  fn select<'a, I>(&self, rng: &mut impl RngCore, population: &'a [I]) -> &'a I
    where I: Individual;
}

#[derive(Default)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
  pub fn new() -> Self {
    Self
  }
}

impl SelectionMethod for RouletteWheelSelection {
  fn select<'a, I>(&self, rng: &mut impl RngCore, population: &'a [I]) -> &'a I
    where I: Individual {
    assert!(!population.is_empty());
    population
      .choose_weighted(rng, |individual| individual.fitness())
      .expect("Error - empty population, or fitnesses are 0")
  }
}

pub trait CrossoverMethod {
  fn crossover(&self, rng: &mut impl RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome;
}

#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;

impl UniformCrossover {
  pub fn new() -> Self {
    Self
  }
}

impl CrossoverMethod for UniformCrossover {
  fn crossover(&self, rng: &mut impl RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome {
    assert_eq!(parent_a.len(), parent_b.len());

    parent_a
      .iter()
      .zip(parent_b.iter())
      .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
      .collect()
  }
}

pub trait MutationMethod {
  fn mutate(&self, rng: &mut impl RngCore, child: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
  chance: f64, // chance of change
  mag: f64 // magnitude of change
}

impl GaussianMutation {
  pub fn new(chance: f64, mag: f64) -> Self {
    assert!(chance >= 0.0 && chance <= 1.0);
    Self { chance, mag }
  }
}

impl MutationMethod for GaussianMutation {
  fn mutate(&self, rng: &mut impl RngCore, child: &mut Chromosome) {
    for gene in child.iter_mut() {
      if rng.gen_bool(self.chance as _) {
        *gene += rng.gen_range(-self.mag..=self.mag);
      }
    }
  }
}

#[derive(Clone, Debug)]
pub struct Statistics {
  min_fitness: f64,
  max_fitness: f64,
  avg_fitness: f64
}

impl Statistics {
  fn new<I>(population: &[I]) -> Self
    where I: Individual {
    assert!(!population.is_empty());

    let mut min_fitness = population[0].fitness();
    let mut max_fitness = min_fitness;
    let mut sum_fitness = 0.0;

    for individual in population {
      let fitness = individual.fitness();

      min_fitness = min_fitness.min(fitness);
      max_fitness = max_fitness.max(fitness);
      sum_fitness += fitness;
    }

    Self {
      min_fitness,
      max_fitness,
      avg_fitness: sum_fitness / (population.len() as f64)
    }
  }

  pub fn min_fitness(&self) -> f64 {
    self.min_fitness
  }

  pub fn max_fitness(&self) -> f64 {
    self.max_fitness
  }

  pub fn avg_fitness(&self) -> f64 {
    self.avg_fitness
  }
}

pub struct GeneticAlgorithm<S, C, M> {
  selection_method: S,
  crossover_method: C,
  mutation_method: M
}

impl<S, C, M> GeneticAlgorithm<S, C, M> 
  where S: SelectionMethod,
        C: CrossoverMethod,
        M: MutationMethod {

  pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
    Self { selection_method, crossover_method, mutation_method }
  }

  pub fn evolve<I: Individual>(&self, rng: &mut impl RngCore, population: &[I]) -> (Vec<I>, Statistics) {
    assert!(!population.is_empty());

    let new_pop = (0..population.len())
      .map(|_| {
        // selection
        let parent_a = self
          .selection_method
          .select(rng, population)
          .as_ref();
        let parent_b = self
          .selection_method
          .select(rng, population)
          .as_ref();

        // crossover
        let mut child = self
          .crossover_method
          .crossover(rng, parent_a, parent_b);

        // mutation
        self.mutation_method.mutate(rng, &mut child);

        I::from(child)
      })
      .collect();
    let stats = Statistics::new(population);
    
    (new_pop, stats)
  }
}

#[derive(Clone, Debug)]
pub struct Chromosome {
  genes: Vec<f64>
}

impl Chromosome {
  pub fn len(&self) -> usize {
    self.genes.len()
  }

  pub fn iter(&self) -> impl Iterator<Item = &f64> {
    self.genes.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
    self.genes.iter_mut()
  }
}

impl Index<usize> for Chromosome {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    &self.genes[index]
  }
}

impl FromIterator<f64> for Chromosome {
  fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
    Self { genes: iter.into_iter().collect() }
  }
}

impl IntoIterator for Chromosome {
  type Item = f64;
  type IntoIter = impl Iterator<Item = f64>;

  fn into_iter(self) -> Self::IntoIter {
    self.genes.into_iter()
  }
}