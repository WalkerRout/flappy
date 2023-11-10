use rand::RngCore;

use crate::*;

#[derive(Debug, Clone)]
pub struct Brain {
  pub(crate) nn: nn::Network,
}

impl Brain {
  pub(crate) fn random(rng: &mut impl RngCore) -> Self {
    Self { 
      nn: nn::Network::random(rng, &Self::topology()), 
    }
  }

  pub(crate) fn chromosome(&self) -> ga::Chromosome {
    self.nn.weights().collect()
  }

  fn topology() -> [nn::LayerArchitecture; 4] {
    [5, 5, 5, 1].map(Into::into)
  }
}

impl From<ga::Chromosome> for Brain {
  fn from(chromosome: ga::Chromosome) -> Self {
    Self { 
      nn: nn::Network::from_weights(&Self::topology(), chromosome), 
    }
  }
}