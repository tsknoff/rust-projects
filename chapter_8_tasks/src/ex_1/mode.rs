use std::collections::HashMap;

pub fn mode(vector: &[i32]) -> i32 {
    let mut hash_map = HashMap::new();

    for key in vector {
        let count = hash_map.entry(key).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;

    for (key, value) in hash_map {
        if value > max {
            max = value;
            mode = *key;
        }
    }

    mode
}