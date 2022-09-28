use std::vec;

mod simulation;
mod reaction;

use simulation::Simulation;


const V: f32 = 1e-15;
const NA: f32 = 6.022e23;
const kf: f32 = 1.07e5 / (NA * V);
const kr: f32 = 0.351;


fn main() {
    let n_a = 1000.;
    let n_b = 1000.;
    let n_c = 0.;
    let total_time: f32 = 30.;

    let mut r1 = kf * n_a * n_b;
    let mut r2 = kr * n_c;

    let mut simulation = Simulation::new();
    simulation.add_species(&String::from("A"), n_a);
    simulation.add_species(&String::from("B"), n_b);
    simulation.add_species(&String::from("C"), n_c);

    simulation.add_reaction(vec!["A".to_string(), "B".to_string()], vec!["C".to_string()], kf);  // OMG
    simulation.add_reaction(vec!["C".to_string()], vec!["A".to_string(), "B".to_string()], kr);

    simulation.simulate();

}
