use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

use crate::reaction::Reaction;

pub struct Simulation {
    total_time: f32, // For now, total time and delta are in the same units. TODO: Make a trait with time units
    delta: f32,
    species: HashMap<String, f32>,
    reactions: Vec<Reaction>,
    rates_vector: Vec<f32>,
    propensity_vector: Vec<f32>,
    total_rate_sum: f32,
    pub species_history: Vec<f32>
}

/*

ALL THE LOOPS HERE CAN BE REPLACED IF I USE MATRICES INSTEAD OF ALL THESE STRUCTS.
USE THE FACT THAT REACTANTS AND PRODUCTS <= 2

*/

impl Simulation {
    pub fn new(time: f32) -> Self {
        return Simulation {
            total_time: time,
            delta: 0.,
            species: HashMap::new(),
            reactions: Vec::new(),
            rates_vector: Vec::new(),
            propensity_vector: Vec::new(),
            total_rate_sum: 0.,
            species_history: Vec::new()
        }
    }

    pub fn add_reaction(&mut self, reactants: Vec<String>, products: Vec<String>, k: f32) {
        let mut reaction = Reaction::new(reactants, products, k);
        let rate = self.calculate_rate_reaction(&mut reaction);
        self.total_rate_sum += rate;
        self.rates_vector.push(rate);
        
        self.reactions.push(reaction);
        self.propensity_vector.push(0.);
    }
    
    pub fn add_species(&mut self, particle_name: String, particle_number: f32) {
        self.species.insert(particle_name, particle_number);
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
            println!("{}",self.propensity_vector[i]);
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
        self.prepare_propensities(true);
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
            self.reactions[reaction_num].react(&mut self.species);
            self.update_rates();

            self.prepare_propensities(false);
            println!("A: {}, B: {}, C: {}", self.species["A"], self.species["B"], self.species["C"]);
            self.species_history.push(self.species["A"]);            
        }
    }
}