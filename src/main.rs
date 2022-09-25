use std::vec;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use ndarray::{arr2, Array2, arr1, Array1, s};


const V: f32 = 1e-15;
const NA: f32 = 6.022e23;
const kf: f32 = 1.07e5 / (NA * V);
const kr: f32 = 0.351;


fn main() {
    let n_a = 1000.;
    let n_b = 1000.;
    let n_c = 0.;
    let total_time: f32 = 30.;

    let mut t: f32 = 0.;

    let mut r1 = kf * n_a * n_b;
    let mut r2 = kr * n_c;
    let mut rate_sum = r1 + r2;
    let mut rates = vec![r1 / rate_sum, r2 / rate_sum];
    let mut propensity = vec![rates[0], rates[1] + rates[0], 1.];
    let reactions: Array2<f32> = arr2(&[[-1., -1., 1.], [1., 1., -1.], [0., 0., 0.]]);
    let mut species: Array1<f32> = arr1(&[n_a, n_b, n_c]);


    let distribution: Uniform<f32> = Uniform::new(0., 1.);
    let mut rng = thread_rng();

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
