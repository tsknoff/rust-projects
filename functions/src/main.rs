use std::io;

fn main() {
    println!("Main function");
    another_function(5);
}

fn another_function(x: i32) {
    println!("Another function: {}", x);
}
