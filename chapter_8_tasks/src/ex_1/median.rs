pub fn median(vector: &[i32]) -> f64 {
    if vector.len() == 0 {
        return 0.0;
    }

    let mid = vector.len() / 2;

    if vector.len() % 2 == 0 {
        (vector[mid] as f64 + vector[mid - 1] as f64) / 2.0
    } else {
        vector[mid] as f64
    }
}