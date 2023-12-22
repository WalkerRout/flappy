
use rand::{Rng, RngCore};

#[derive(Clone, Debug)]
pub struct Network {
  layers: Vec<Layer>,
}

impl Network {
  pub fn random(rng: &mut impl RngCore, layers: &[LayerArchitecture]) -> Self {
    assert!(layers.len() > 1);

    let layers = layers
      .windows(2)
      .map(|layers| {
        Layer::random(rng, layers[0].neurons, layers[1].neurons)
      })
      .collect();

    Self { layers }
  }

  pub fn from_weights(layers: &[LayerArchitecture], weights: impl IntoIterator<Item = f64>) -> Self {
    assert!(layers.len() > 1);

    let mut weights = weights.into_iter();

    let layers = layers
      .windows(2)
      .map(|layers| {
        Layer::from_weights(
          layers[0].neurons,
          layers[1].neurons,
          &mut weights,
        )
      })
      .collect();

    if weights.next().is_some() {
      panic!("Error - too many weights");
    }

    Self { layers }
  }

  pub fn propagate(&self, inputs: Vec<f64>) -> Vec<f64> {
    let layers = self.layers.len();
    let last = &self.layers[layers - 1];
    let activation      = |x: f64| x.max(0.0);
    let activation_last = |x: f64| 1.0 / (1.0 + (-x).exp());

    let hidden_output = self.layers
      .iter()
      .take(layers-1)
      .fold(inputs, |inputs, layer| {
        layer.propagate(inputs)
          .into_iter()
          .map(activation)
          .collect()
      });
      
    last
      .propagate(hidden_output)
      .into_iter()
      .map(activation_last)
      .collect()
  }

  pub fn weights(&self) -> impl Iterator<Item = f64> + '_ {
    self.layers
      .iter()
      .flat_map(|layer| layer.neurons.iter())
      .flat_map(|neuron| std::iter::once(&neuron.bias).chain(&neuron.weights))
      .copied()
  }
}

#[derive(Clone, Copy, Debug)]
pub struct LayerArchitecture {
  pub neurons: usize,
}

impl From<usize> for LayerArchitecture {
  fn from(neurons: usize) -> Self {
    Self { neurons }
  }
}

#[derive(Clone, Debug)]
struct Layer {
  neurons: Vec<Neuron>,
}

impl Layer {
  pub fn random(rng: &mut impl RngCore, input: usize, output: usize) -> Self {
    let neurons = (0..output)
      .map(|_| Neuron::random(rng, input))
      .collect();

    Self { neurons }
  }

  pub fn from_weights(input_size: usize, output_size: usize, weights: &mut impl Iterator<Item = f64>) -> Self {
    let neurons = (0..output_size)
      .map(|_| Neuron::from_weights(input_size, weights))
      .collect();

    Self { neurons }
  }

  fn propagate(&self, inputs: Vec<f64>) -> Vec<f64> {
    self.neurons
      .iter()
      .map(|neuron| neuron.propagate(&inputs))
      .collect()
  }
}

#[derive(Clone, Debug)]
struct Neuron {
  bias: f64,
  weights: Vec<f64>,
}

impl Neuron {
  pub fn random(rng: &mut impl RngCore, input_size: usize) -> Self {
    let bias = rng.gen_range(-1.0..=1.0);

    let weights = (0..input_size)
      .map(|_| rng.gen_range(-1.0..=1.0))
      .collect();

    Self { bias, weights }
  }

  pub fn from_weights(input_size: usize, weights: &mut impl Iterator<Item = f64>) -> Self {
    let bias = weights.next().expect("Error - not enough weights");

    let weights = (0..input_size)
      .map(|_| weights.next().expect("Error - not enough weights"))
      .collect();

    Self { bias, weights }
  }

  fn propagate(&self, inputs: &[f64]) -> f64 {
    assert_eq!(inputs.len(), self.weights.len());
    
    let output = inputs
      .iter()
      .zip(self.weights.iter())
      .map(|(i, w)| i * w)
      .sum::<f64>();
    
    output + self.bias
  }
}
