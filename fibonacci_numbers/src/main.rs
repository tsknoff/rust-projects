fn main() {
    let n: u32 = 3;
    println!("{}th Fibonacci number is: {}", n, get_n_fibonacci_number(n));
}

fn get_n_fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return get_n_fibonacci_number(n - 1) + get_n_fibonacci_number(n - 2);
    }
}