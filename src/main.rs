use std::vec;
use plotters::prelude::*;

mod simulation;
mod reaction;
use simulation::Simulation;


const V: f32 = 1e-15;
const NA: f32 = 6.022e23;
const Kf: f32 = 1.07e5 / (NA * V);
const Kr: f32 = 0.351;
const OUT_FILE_NAME: &'static str = "sample.png";


fn main() {
    let n_a = 1000.;
    let n_b = 1000.;
    let n_c = 0.;
    let total_time: f32 = 5.;

    let mut simulation = Simulation::new(total_time);
    simulation.add_species("A".to_string(), n_a);
    simulation.add_species("B".to_string(), n_b);
    simulation.add_species("C".to_string(), n_c);

    simulation.add_reaction(vec!["A".to_string(), "B".to_string()], vec!["C".to_string()], Kf);  // OMG
    simulation.add_reaction(vec!["C".to_string()], vec!["A".to_string(), "B".to_string()], Kr);

    simulation.simulate();

    plot(simulation);
}

fn plot(simulation: Simulation) {
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (600, 400))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..simulation.species_history.len(), 0.0..1000.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..).zip(simulation.species_history.iter().map(|x | *x as f64)), &GREEN)
    ).unwrap();
}