pub struct Reaction<'a> {
    reactants: Vec<&'a Species>,
    products: Vec<&'a Species>,
    st_vec: Vec<i32>,
    rate: f32
}

#[derive(Clone)]
pub struct Species {
    name: String,
    n: f32,
}

impl<'a> Reaction<'a> {
    pub fn new(reactants: Vec<&'a Species>, products: Vec<&'a Species>, rate: f32) -> Self {
        // reactants and products should be species already here.
        let mut st_vec: Vec<i32> = Vec::new();
        let mut reactants_particle = Vec::new();
        let mut products_particle = Vec::new();

        for i in 0..reactants.len() {
            st_vec.push(-1);
            reactants_particle.push(reactants[i]);
        }
        for i in 0..products.len() {
            st_vec.push(1);
            products_particle.push(products[i]);
        }
        return Self {
            reactants: reactants_particle,
            products: products_particle,
            st_vec: st_vec,
            rate: rate
        }
    }
}

impl Species {
    pub fn get_n(&self) -> f32 { self.n }
    pub fn get_name(&self) -> &String { &self.name }
}