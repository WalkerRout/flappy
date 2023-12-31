use serde::Serialize;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;

use std::panic;

use lib_simulation as sim;

#[derive(Clone, Debug, Serialize)]
pub struct Bird {
  pub x: f64,
  pub y: f64,
  pub dy: f64,
  pub offx: f64,
  pub offy: f64,
  pub fitness: f64,
}

impl From<&sim::Bird> for Bird {
  fn from(bird: &sim::Bird) -> Self {
    Self {
      x: bird.position().x,
      y: bird.position().y,
      dy: bird.dy(),
      offx: sim::BIRD_OFFSET_X,
      offy: sim::BIRD_OFFSET_Y,
      fitness: bird.fit_distance(),
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct Pipe {
  pub x: f64,
  pub y: f64,
  pub offx: f64,
  pub offy: f64,
}

impl From<&sim::Pipe> for Pipe {
  fn from(pipe: &sim::Pipe) -> Self {
    Self {
      x: pipe.position().x,
      y: pipe.position().y,
      offx: sim::PIPE_OFFSET_X,
      offy: sim::PIPE_OFFSET_Y,
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
  pub birds: Vec<Bird>,
  pub pipes: Vec<Pipe>
}

impl From<&sim::World> for World {
  fn from(world: &sim::World) -> Self {
    let birds = world
      .birds()
      .map(Bird::from)
      .collect();

    let pipes = world
      .pipes()
      .map(Pipe::from)
      .collect();

    Self { 
      birds,
      pipes
    }
  }
}

#[wasm_bindgen]
pub struct Simulation {
  rng: ThreadRng,
  sim: sim::Simulation
}

#[wasm_bindgen]
impl Simulation {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut rng = thread_rng();
    let sim = sim::Simulation::random(&mut rng);

    Self { rng, sim }
  }

  pub fn step(&mut self) -> bool {
    self.sim.step(&mut self.rng)
  }

  pub fn train(&mut self) -> bool {
    self.sim.train(&mut self.rng);
    self.sim.tick_count() == 0
  }

  pub fn world(&self) -> JsValue {
    let world = World::from(self.sim.world());
    <JsValue as JsValueSerdeExt>::from_serde(&world).unwrap()
  }

  pub fn ticks(&self) -> JsValue {
    JsValue::from_f64(self.sim.tick_count() as f64)
  }
}

impl Default for Simulation {
  fn default() -> Self {
    Self::new()
  }
}