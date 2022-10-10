use std::collections::HashMap;

pub fn max_hashmap(h: &HashMap<String, Vec<f32>>) -> f32 {
    let mut max_val = 0.; // -inf
    for (_, vect) in h {
        for v in vect {
            if *v > max_val {
                max_val = *v;
            }
        }
    }
    max_val
}

pub fn max_hashmap_constrained(h: &HashMap<String, Vec<f32>>, constraint: &Vec<&str>) -> f32 {
    let mut max_val = 0.; // -inf
    for (k, vect) in h {
        if constraint.contains(&k.as_str()) {
            for v in vect {
                if *v > max_val {
                    max_val = *v;
                }
            }
        }
    }
    max_val
}

pub fn split_whitespace(s: &str) -> Vec<&str> {
    let words: Vec<&str> = s.split_whitespace().collect();
    return words;
}