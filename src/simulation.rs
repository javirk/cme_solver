use std::borrow::Borrow;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

use crate::reaction::{Reaction, Species};

pub struct Simulation {
    total_time: f32, // For now, total time and delta are in the same units. TODO: Make a trait with time units
    delta: f32,
    species: Vec<Species>,
    reactions: Vec<Reaction>,
    propensity_vector: Vec<f32>,
    total_rate_sum: f32
}

impl Simulation {
    pub fn new() -> Self {
        return Simulation {
            total_time: 30.,
            delta: 0.,
            species: Vec::new(),
            reactions: Vec::new(),
            propensity_vector: Vec::new(),
            total_rate_sum: 0.
        }
    }

    pub fn add_reaction(&mut self, reactants: Vec<String>, products: Vec<String>, rate: f32) {
        let mut reactants_spec = Vec::new();
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

    pub fn add_species(&mut self, particle_name: &str, particle_number: f32) {
        self.species.push(Species {
            name: particle_name.into(),
            n: particle_number
        })
    }

    fn update_propensities(&mut self) {

    }

    pub fn simulate(&mut self) {
        self.prepare_propensities();
        let distribution: Uniform<f32> = Uniform::new(0., 1.);
        let mut rng = thread_rng();

        let mut t: f32 = 0.;

        while t < total_time {
            let a = rng.sample(distribution);
            let tau = (1. / rate_sum) * (1./a).ln();
            t = t + tau;
            
            let mu: f32 = rng.sample(distribution);
            
            let mut reaction_num: usize = 0;
            for i in 0..propensity.len() {
                if mu < propensity[i] {
                    reaction_num = i;
                    break;
                }
            }
            
            // Do a reaction
            let rr = reactions.slice(s![reaction_num, ..]);
            species = species + rr;
            println!("{}", species);
    
            // Update
            r1 = kf * species[0] * species[1];
            r2 = kr * species[2];
            rate_sum = r1 + r2;
            rates = vec![r1 / rate_sum, r2 / rate_sum];
            propensity = vec![rates[0], rates[1] + rates[0], 1.];
        }

    }

}