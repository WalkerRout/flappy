use rand::RngCore;

use crate::*;

#[derive(Debug, Clone)]
pub struct Brain {
  pub(crate) nn: nn::Network,
}

impl Brain {
  pub(crate) fn random(rng: &mut impl RngCore) -> Self {
    Self { nn: nn::Network::random(rng, &Self::topology()), }
  }

  pub(crate) fn chromosome(&self) -> ga::Chromosome {
    self.nn.weights().collect()
  }

  pub(crate) fn from_chromosome(chromosome: ga::Chromosome) -> Self {
    Self { nn: nn::Network::from_weights(&Self::topology(), chromosome), }
  }

  fn topology() -> [nn::LayerArchitecture; 4] {
    [
      nn::LayerArchitecture { neurons: 5 },
      nn::LayerArchitecture { neurons: 5 },
      nn::LayerArchitecture { neurons: 5 },
      nn::LayerArchitecture { neurons: 1 },
    ]
  }
}