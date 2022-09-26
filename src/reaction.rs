struct Reaction {
    reactants: Vec<Species>,
    products: Vec<Species>,
    st_vec: Vec<i32>,
    rate: f32
}

struct Species {
    name: String,
    n: f32,
}

impl Reaction {
    pub fn new(reactants: Vec<String>, products: Vec<String>, rate: f32) -> Self {
        // reactants and products should be species already here.
        let mut st_matrix = [0; reactants.len() + products.len()];
        let mut reactants_particle = [];
        let mut products_particle = [];
        for i in 0..reactants.len() {
            st_matrix[i] -= 1;
            reactants_particle.push(Species { // Don't create Species again!
                name: reactants[i],
                n: 0.
            })
        }
        for i in 0..products.len() {
            st_matrix[reactants.len() + i] += 1;
            products_particle.push(Species {
                name: products[i],
                n: 0.
            })
        }
        return Reaction {
            reactants: reactants_particle,
            products: products_particle,
            st_matrix: st_matrix,
            rate: rate
        }
    }
}