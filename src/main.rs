use std::vec;

use ndarray::{arr2, Array2, arr1, Array1, s};
mod simulation;
mod reaction;

use reaction::Reaction;
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

    simulation.add_reaction(vec!["A".to_string(), "B".to_string()], vec!["C".to_string()], r1);  // OMG
    simulation.add_reaction(vec!["C".to_string()], vec!["A".to_string(), "B".to_string()], r2);








    let mut rate_sum = r1 + r2;
    let mut rates = vec![r1 / rate_sum, r2 / rate_sum];
    let mut propensity = vec![rates[0], rates[1] + rates[0], 1.];
    let reactions: Array2<f32> = arr2(&[[-1., -1., 1.], [1., 1., -1.], [0., 0., 0.]]);
    let mut species: Array1<f32> = arr1(&[n_a, n_b, n_c]);


    let distribution: Uniform<f32> = Uniform::new(0., 1.);
    let mut rng = thread_rng();



}
