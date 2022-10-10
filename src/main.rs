use std::vec;
mod simulation;
mod reaction;
mod utils;
use simulation::Simulation;


const V: f32 = 1e-15;
const NA: f32 = 6.022e23;
const SCALAR: f32 = 2.076e-9;


fn main() {
    let mut sim = Simulation::from_file("saved_simulations/lac.sim").unwrap();

    sim.simulate();

    sim.plot(vec!["A", "C"]);
}