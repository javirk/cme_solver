struct Reaction {
    reactants: Vec<String>,
    products: Vec<String>,
    st_vec: Vec<i32>
}

impl Reaction {
    pub fn new(reactants: Vec<String>, products: Vec<String>) -> Self {
        let mut st_matrix = [0; reactants.len() + products.len()];
        for i in 0..reactants.len() {
            st_matrix[i] -= 1;
        }
        for i in 0..products.len() {
            st_matrix[reactants.len() + i] += 1;
        }
        return Reaction {
            reactants: reactants,
            products: products,
            st_matrix: st_matrix
        }
    }
}