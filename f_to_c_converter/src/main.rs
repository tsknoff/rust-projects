use std::io;

fn main() {
    let mut temperature: String = String::new();

    println!("Enter temperature in F:");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    println!("Readed temperature in F: {temperature}");
    println!("In C^: {}", (temperature-32.0) * 5.0/9.0);
}
