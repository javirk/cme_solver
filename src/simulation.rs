use std::borrow::Borrow;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

use crate::reaction::{Reaction, Species};

pub struct Simulation {
    total_time: f32, // For now, total time and delta are in the same units. TODO: Make a trait with time units
    delta: f32,
    species: HashMap<String, f32>,
    reactions: Vec<Reaction>,
    propensity_vector: Vec<f32>,
    total_rate_sum: f32
}

/*

ALL THE LOOPS HERE CAN BE REPLACED IF I USE MATRICES INSTEAD OF ALL THESE STRUCTS.
USE THE FACT THAT REACTANTS AND PRODUCTS <= 2

*/

impl Simulation {
    pub fn new() -> Self {
        return Simulation {
            total_time: 30.,
            delta: 0.,
            species: HashMap::new(),
            reactions: Vec::new(),
            propensity_vector: Vec::new(),
            total_rate_sum: 0.
        }
    }

    pub fn add_reaction(&mut self, reactants: Vec<String>, products: Vec<String>, k: f32) {
        /*let mut reactants_spec = Vec::new();
        for r in reactants {
            for spec in &self.species {
                if spec.get_name().borrow() == r {
                    reactants_spec.push(spec.borrow().clone()); // OMG, that .borrow().clone()
                }
            }

        }
        let mut products_spec = Vec::new();
        for p in products {
            for spec in &self.species {
                if spec.get_name().borrow() == p {
                    products_spec.push(spec.borrow().clone());
                }
            }
        }*/

        // Reactants and products should be vecs of Species already. So bring them as String but find the proper Species
        let reaction = Reaction::new(reactants, products, k);
        self.total_rate_sum += reaction.rate;
        self.reactions.push(reaction);
        self.propensity_vector.push(0.);
    }

    fn prepare_propensities(&mut self) {
        // This is called before starting the simulation
        for r in 0..self.reactions.len() {
            if r == 0 {
                self.propensity_vector[r] = self.reactions[r].rate / self.total_rate_sum;
            } else {
                self.propensity_vector[r] = self.reactions[r].rate / self.total_rate_sum + self.propensity_vector[r-1];
            }
            println!("{}",self.propensity_vector[r]);
        }
        self.propensity_vector.push(1.);
    }

    pub fn add_species(&mut self, particle_name: String, particle_number: f32) {
        self.species.insert(particle_name, particle_number);
    }

    fn update_propensities(&mut self) {
        let mut total_rate_sum = 0.;
        for i in 0..self.reactions.len() {
            total_rate_sum += self.reactions[i].rate;
        }

        for r in 0..self.reactions.len() {
            if r == 0 {
                self.propensity_vector[r] /= self.total_rate_sum;
            } else {
                self.propensity_vector[r] = (self.propensity_vector[r] + self.propensity_vector[r-1]) / self.total_rate_sum;
            }
        }

    }

    pub fn simulate(&mut self) {
        self.prepare_propensities();
        let distribution: Uniform<f32> = Uniform::new(0., 1.);
        let mut rng = thread_rng();

        let mut t: f32 = 0.;

        while t < self.total_time {
            let a = rng.sample(distribution);
            let tau = (1. / self.total_rate_sum) * (1./a).ln();
            t = t + tau;
            
            let mu: f32 = rng.sample(distribution);
            
            let mut reaction_num: usize = 0;
            for i in 0..self.propensity_vector.len() - 1 {
                if mu < self.propensity_vector[i] {
                    reaction_num = i;
                    break;
                }
            }
            
            // Do a reaction
            self.reactions[reaction_num].react();

            self.update_propensities();
            println!("A: {}, B: {}, C: {}", self.species[0].get_n(), self.species[1].get_n(), self.species[2].get_n());
            
            // println!("{}", species);
        }

    }

}