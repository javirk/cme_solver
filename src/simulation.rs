use std::borrow::Borrow;

use crate::reaction::{Reaction, Species};

pub struct Simulation<'a> {
    total_time: f32, // For now, total time and delta are in the same units. TODO: Make a trait with time units
    delta: f32,
    species: Vec<Species>,
    reactions: Vec<Reaction<'a>>,
    propensity_vector: Vec<f32>,
    total_rate_sum: f32
}

impl Simulation<'_> {
    fn new() -> Self {
        return Simulation {
            total_time: 30.,
            delta: 0.,
            species: Vec::new(),
            reactions: Vec::new(),
            propensity_vector: Vec::new(),
            total_rate_sum: 0.
        }
    }

    fn add_reaction(&mut self, reactants: Vec<String>, products: Vec<String>, rate: f32) {
        let mut reactants_spec = Vec::new();
        for r in reactants {
            for spec in &self.species {
                if spec.get_name().borrow() == r {
                    reactants_spec.push(spec);
                }
            }
        }
        let mut products_spec = Vec::new();
        for p in products {
            for spec in &self.species {
                if spec.get_name().borrow() == p {
                    products_spec.push(spec);
                }
            }
        }

        // Reactants and products should be vecs of Species already. So bring them as String but find the proper Species
        let reaction = Reaction::new(reactants_spec, products_spec, rate);
        self.reactions.push(reaction);
        self.propensity_vector.push(0.);
        self.total_rate_sum += rate;
    }

    fn prepare_propensities(&mut self) {
        // This is called before starting the simulation
        for r in 0..self.reactions.len() {
            if r == 0 {
                self.propensity_vector[r] /= self.total_rate_sum;
            } else {
                self.propensity_vector[r] = (self.propensity_vector[r] + self.propensity_vector[r-1]) / self.total_rate_sum;
            }
        }
        self.propensity_vector.push(1.);
    }

    fn add_species(&mut self, particle_name: String, particle_number: f32) {
        self.species.push(Species {
            name: particle_name,
            n: particle_number
        })
    }

    fn update_propensities(&mut self) {

    }
}