fn main() {
    // Vectors are similar to arrays but can grow or shrink in size
    let vector_1: Vec<i32> = Vec::new();
    println!("{:?}", vector_1);
    println!();

    // The vec! macro can be used to create a vector with initial values
    let vector_2 = vec![1, 2, 3];
    println!("{:?}", vector_2);
    println!();

    // Vectors can be updated by pushing values to them
    let mut vector_3 = Vec::new();
    vector_3.push(4);
    vector_3.push(5);
    vector_3.push(6);
    vector_3.push(7);
    println!("{:?}", vector_3);
    println!();

    // Vectors can be accessed using indexing
    let third: Option<&i32> = vector_3.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    println!();

    // Vectors can be iterated over
    let vector_4 = vec![100, 32, 57];
    for i in &vector_4 {
        println!("{i}");
    }
    println!();

    // Vectors can be iterated over and modified
    let mut vector_5 = vec![100, 32, 57];
    for i in &mut vector_5 {
        *i += 50;
    }
    println!("{:?}", vector_5);
    println!();

    // Vectors can store values of different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
        }
    }
    println!();
}
