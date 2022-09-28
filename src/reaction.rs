use std::collections::HashMap;

pub struct Reaction {
    reactants: Vec<String>,
    products: Vec<String>,
    st_vec: Vec<f32>,
    k: f32,
    pub rate: f32
}

#[derive(Clone)]
pub struct Species {
    pub name: String,
    pub n: f32,
}

impl Reaction {
    pub fn new(reactants: Vec<String>, products: Vec<String>, mut k: f32) -> Self {
        let mut st_vec: Vec<f32> = Vec::new();
        let mut reactants_particle = Vec::new();
        let mut products_particle = Vec::new();
        let mut rate: f32 = k;

        for i in 0..reactants.len() {
            st_vec.push(-1.);
            reactants_particle.push(reactants[i].clone());
            rate *= reactants[i].get_n();
        }
        for i in 0..products.len() {
            st_vec.push(1.);
            products_particle.push(products[i].clone());
        }
        return Self {
            reactants: reactants_particle,
            products: products_particle,
            st_vec: st_vec,
            k: k,
            rate: rate
        }
    }

    pub fn react(&mut self) { // This function is really ugly. I'm sure it can be improved --> Make matrices instead of Species
        for i in 0..self.reactants.len() {
            self.reactants[i].n += self.st_vec[i];
        }
        for i in 0..self.products.len() {
            self.products[i].n += self.st_vec[i + self.reactants.len()];
        }

        self.update_rates();
    }

    fn update_rates(&mut self) {
        self.rate = self.k;
        for i in 0..self.reactants.len() {
            self.rate *= self.reactants[i].get_n();
        }
    }
}

impl Species {
    pub fn get_n(&self) -> f32 { self.n }
    pub fn get_name(&self) -> &str { &self.name }
}