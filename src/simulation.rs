use std::collections::HashMap;
use std::time::Duration;
use std::fs::File;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use plotters::prelude::*;
use std::io::{BufReader, prelude::*, Error};

use crate::utils;
use crate::reaction::Reaction;

type Result<T> = std::result::Result<T, Error>;

const OUT_FILE_NAME: &'static str = "sample.png";

pub struct Simulation {
    total_time: f32, // For now, total time and delta are in the same units. TODO: Make a trait with time units
    delta: f32,
    pub species: HashMap<String, f32>,
    reactions: Vec<Reaction>,
    rates_vector: Vec<f32>,
    propensity_vector: Vec<f32>,
    total_rate_sum: f32,
    time_history: Vec<f32>,
    species_history: HashMap<String, Vec<f32>>,
    write_interval: f32,
    simulation_method: String,
}

/*

ALL THE LOOPS HERE CAN BE REPLACED IF I USE MATRICES INSTEAD OF ALL THESE STRUCTS.
USE THE FACT THAT REACTANTS AND PRODUCTS <= 2

*/

impl Simulation {
    pub fn new(time: f32, write_interval: f32, simulation_method: &str) -> Self {
        return Simulation {
            total_time: time,
            delta: 0.,
            species: HashMap::new(),
            reactions: Vec::new(),
            rates_vector: Vec::new(),
            propensity_vector: Vec::new(),
            total_rate_sum: 0.,
            time_history: Vec::new(),
            species_history: HashMap::new(),
            write_interval: write_interval,
            simulation_method: simulation_method.to_string()
        }
    }

    pub fn set_delta(&mut self, delta: Duration) {
        self.delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 * 1e-9;
    }

    pub fn add_reaction(&mut self, reactants: Vec<&str>, products: Vec<&str>, k: f32) {
        self.assert_species_reaction(&reactants, &products);
        let mut reaction = Reaction::new(reactants, products, k);
        let rate = self.calculate_rate_reaction(&mut reaction);
        self.total_rate_sum += rate;
        self.rates_vector.push(rate);
        
        self.reactions.push(reaction);
        self.propensity_vector.push(0.);
    }

    fn assert_species_reaction(&self, reactants: &Vec<&str>, products: &Vec<&str>) {
        for v in reactants {
            assert!(self.species.contains_key(*v), "{} Not found in the reactants", *v)
        }
        for v in products {
            assert!(self.species.contains_key(*v), "{} Not found in the products", *v)
        }
    }
    
    pub fn add_species(&mut self, particle_name: &str, particle_number: f32) {
        self.species.insert(particle_name.to_string(), particle_number);
        self.species_history.insert(particle_name.to_string(), Vec::new());
    }

    fn calculate_rate_reaction(&self, reaction: &Reaction) -> f32 {
        let mut rate: f32 = (*reaction).k;
        for r in &reaction.reactants {
            rate *= self.species[r];
        }
        return rate
    }

    fn prepare_propensities(&mut self, first_call: bool) {
        for i in 0..self.rates_vector.len() {
            if i == 0 {
                self.propensity_vector[i] = self.rates_vector[i] / self.total_rate_sum;
            } else {
                self.propensity_vector[i] = self.rates_vector[i] / self.total_rate_sum + self.propensity_vector[i-1];
            }
        }
        if first_call {
            self.propensity_vector.push(1.);
        }
    }

    fn update_rates(&mut self) {
        self.total_rate_sum = 0.;
        for i in 0..self.reactions.len() {
            let r = &self.reactions[i];
            let rate = self.calculate_rate_reaction(&r);
            self.rates_vector[i] = rate;
            self.total_rate_sum += rate;
        }
    }

    pub fn simulate(&mut self) {
        if self.simulation_method == "GillespieSolver".to_string() {
            self.simulate_gillespie();
        } else if self.simulation_method == "NextReaction".to_string() {
            assert!(self.delta > 0., "Using next reaction method and delta <= 0 is not allowed");
            self.simulate_nextreaction();
        } else {
            panic!("Not implemented simulation method {}", self.simulation_method);
        }
    }

    fn simulate_gillespie(&mut self) {
        self.prepare_propensities(true);
        let distribution: Uniform<f32> = Uniform::new(0., 1.);
        let mut rng = thread_rng();

        let mut t: f32 = 0.;
        let mut i: f32 = 0.;

        while t < self.total_time {
            let a = rng.sample(distribution);
            let tau = (1. / self.total_rate_sum) * (1./a).ln();
            t += tau;
            
            let mu: f32 = rng.sample(distribution);
            
            let mut reaction_num: usize = 0;
            for i in 0..self.propensity_vector.len() - 1 {
                if mu < self.propensity_vector[i] {
                    reaction_num = i;
                    break;
                }
            }
            
            // Do a reaction
            self.reactions[reaction_num].react(&mut self.species);
            self.update_rates();

            self.prepare_propensities(false);

            // Write
            i += 1.;
            if i % self.write_interval == 0. {
                self.time_history.push(t);
                for (spec, n) in &self.species {
                    self.species_history.get_mut(spec).unwrap().push(*n);
                }  
            }
            if i % (self.write_interval * 100.) == 0. {
                println!("{}/{}", t, self.total_time);
            }
        }
    }

    fn simulate_nextreaction(&mut self) {
        self.prepare_propensities(true);
        let distribution: Uniform<f32> = Uniform::new(0., 1.);
        let mut rng = thread_rng();

        let mut t: f32 = 0.;
        let mut i: f32 = 0.;

        while t < self.total_time {
            t += self.delta;
            
            let mu: f32 = rng.sample(distribution);
            
            let mut reaction_num: usize = 0;
            for i in 0..self.propensity_vector.len() - 1 {
                if mu < self.propensity_vector[i] {
                    reaction_num = i;
                    break;
                }
            }
            
            // Do a reaction
            self.reactions[reaction_num].react(&mut self.species);
            self.update_rates();

            self.prepare_propensities(false);

            // Write
            i += 1.;
            if i % self.write_interval == 0. {
                self.time_history.push(t);
                for (spec, n) in &self.species {
                    self.species_history.get_mut(spec).unwrap().push(*n);
                }  
            }
            if i % (self.write_interval * 10.) == 0. {
                println!("{}/{}", t, self.total_time);
            }
        }
    }

    pub fn plot(&self, species_plot: Vec<&str>) {
        assert!(species_plot.len() > 0);
        let max_val = utils::max_hashmap_constrained(&self.species_history, &species_plot);
        let root_area = BitMapBackend::new(OUT_FILE_NAME, (600, 400))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Line Plot Demo", ("sans-serif", 40))
            .build_cartesian_2d(0f32..self.time_history[self.time_history.len() - 1], 0f32..max_val)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();
        
        for (idx, history) in (0..).zip(&self.species_history) {
            if species_plot.contains(&history.0.as_str()) {
                ctx.draw_series(
                    //LineSeries::new((0..).zip(history.1.iter().map(|x | (self.time_history, *x as f64))), &Palette99::pick(idx))
                    LineSeries::new(self.time_history.iter().zip(history.1).map(|(x, y)| (*x, *y)), &Palette99::pick(idx))
                ).unwrap();
            }
        }
    }
}

impl Simulation {
    pub fn from_file(sim_file: &str) -> Result<Self> {
        let file = File::open(sim_file)?;//.expect("file not found!");

        let mut simulation = Simulation {
            total_time: 100.,
            delta: 0.,
            species: HashMap::new(),
            reactions: Vec::new(),
            rates_vector: Vec::new(),
            propensity_vector: Vec::new(),
            total_rate_sum: 0.,
            time_history: Vec::new(),
            species_history: HashMap::new(),
            write_interval: 10.,
            simulation_method: "GillespieSolver".to_string()
        };

        let buf_reader = BufReader::new(file);
      
        for line in buf_reader.lines() {
            let l = line?;
            if l.len() == 0 {
                continue;
            }
            match &l[..1] {
                "t" | "w" => Simulation::add_basic_line(&mut simulation, &l),
                "d" => Simulation::add_delta_line(&mut simulation, &l),

                "r" => Simulation::add_reaction_line(&mut simulation, &l),
                "s" => Simulation::add_species_line(&mut simulation, &l),
                _ => (),
            }
        }
        println!("Simulation file is loaded");
        Ok(simulation)
    }

    fn add_reaction_line(sim: &mut Simulation, line: &str) {
        let line_replaced = &line.replace("+", "");
        let mut line_vec = utils::split_whitespace(line_replaced);
        line_vec = line_vec[1..].to_vec();

        // Reference: reactants: Vec<&str>, products: Vec<&str>, k: f32
        let mut reactants: Vec<&str> = Vec::new();
        let mut products: Vec<&str> = Vec::new();
        let mut k: f32 = 1.;

        let mut before_arrow: bool = true;

        for value in line_vec {
            match value.parse::<f32>() {
                Ok(new_k) => {
                    k = k*new_k;
                    continue;
                },
                Err(_) => ()
            };
            match value {
                "->" => before_arrow = false,
                "+" => (),
                _ => {
                    if before_arrow {
                        reactants.push(value);
                    } else {
                        products.push(value);
                    }
                }
            }
        }


        sim.add_reaction(reactants, products, k);
    }

    fn add_species_line(sim: &mut Simulation, line: &str) {
        let line_vec = utils::split_whitespace(line);
        assert!(line_vec.len() >= 3);
        let species_name = line_vec[1];
        let mut species_number: f32 = 1.;

        for v in &line_vec[2..] {
            species_number *= v.parse::<f32>().unwrap();
        }

        sim.add_species(species_name, species_number);
    }

    fn add_basic_line(sim: &mut Simulation, line: &str) {
        let line_vec = utils::split_whitespace(line);
        assert!(line_vec.len() == 2);
        match line_vec[0] {
            "t" => sim.total_time = line_vec[1].parse::<f32>().unwrap(), // This can give an error
            "w" => sim.write_interval = line_vec[1].parse::<f32>().unwrap(),
            _ => ()
        }
    }

    fn add_delta_line(sim: &mut Simulation, line: &str) {
        let line_vec = utils::split_whitespace(line);
        assert!(line_vec.len() == 3);
        let value = line_vec[2].parse::<u64>().unwrap();
        let delta_time = match line_vec[1] {
            "m" => Duration::from_millis(value),
            "n" => Duration::from_nanos(value),
            "s" => Duration::from_secs(value),
            _ => Duration::from_secs(1),
        };
        sim.set_delta(delta_time);
    }

}