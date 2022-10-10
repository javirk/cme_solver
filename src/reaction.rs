use std::collections::HashMap;

pub struct Reaction {
    pub reactants: Vec<String>,
    products: Vec<String>,
    st_vec: Vec<f32>,
    pub k: f32,
}

impl Reaction {
    pub fn new(reactants: Vec<&str>, products: Vec<&str>, k: f32) -> Self {
        let mut st_vec: Vec<f32> = Vec::new();
        let mut reactants_string = Vec::new();
        let mut products_string = Vec::new();

        for i in 0..reactants.len() {
            reactants_string.push(reactants[i].to_string());
            st_vec.push(-1.);
        }
        for i in 0..products.len() {
            products_string.push(products[i].to_string());
            st_vec.push(1.);
        }
        return Self {
            reactants: reactants_string,
            products: products_string,
            st_vec: st_vec,
            k: k
        }
    }

    pub fn react(&mut self, species: &mut HashMap<String, f32>) { // This function is really ugly. I'm sure it can be improved --> Make matrices instead of Species
        for (i, r) in self.reactants.iter().enumerate() {
            *species.get_mut(r).unwrap() += self.st_vec[i];
        }
        for (i, r) in self.products.iter().enumerate() {
            *species.get_mut(r).unwrap() += self.st_vec[i + self.reactants.len()];
        }
    }
}