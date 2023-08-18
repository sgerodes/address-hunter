pub mod address_utils {
    use std::collections::HashMap;

    pub fn calculate_proximity_coefficient(address: &str, proximity_max_distance: usize) -> f64 {
        let mut proximity_coefficient = 0.0;
        let mut indexes: HashMap<char, Vec<usize>> = HashMap::new();
    
        for (i, c) in address.chars().enumerate() {
            indexes.entry(c).or_default().push(i);
        }
    
        for c_indexes in indexes.values() {
            for (ii, &first_value) in c_indexes.iter().enumerate() {
                for j in 1..=proximity_max_distance {
                    let ii2 = ii + j;
                    if ii2 < c_indexes.len() {
                        let second_value = c_indexes[ii2];
                        let distance = second_value as f64 - first_value as f64;
                        proximity_coefficient += 1.0 / distance;
                    }
                }
            }
        }
    
        proximity_coefficient
    }

    pub fn calculate_entropy(address: &str) -> f64 {
        let mut char_freq = HashMap::new();
    
        for c in address.chars() {
            *char_freq.entry(c).or_insert(0u32) += 1;
        }
    
        let total_chars = address.len() as f64;
        let char_probs: Vec<f64> = char_freq.values().map(|&count| count as f64 / total_chars).collect();
        
        -char_probs.iter().fold(0.0, |acc, &p| acc + p * p.log2())
    }
}