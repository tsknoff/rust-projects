pub fn average(vector: &[i32]) -> f64 {
    if vector.len() == 0 {
        return 0.0;
    }

    let sum: i32 = vector.iter().sum();
    sum as f64 / vector.len() as f64
}