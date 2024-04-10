mod stats;

fn main() {
    // Create a new string
    //let mut s = String::new(); // warning: unused variable: `s`

    // let data = "initial contents";
    // let s = data.to_string();
    // the method also works on a literal directly:
    // let s = "initial contents".to_string();
    // from a string literal
    // let s = String::from("initial contents");

    // Appending to a string with push_str and push
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push_str takes a string slice
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // fn add(self, s: &str) -> String
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("s3 is {s3}");
    println!();

    // If we need to concatenate multiple strings, the + operator becomes unwieldy
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! macro
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");
    println!();

    // Indexing into Strings
    let s1 = String::from("hello");
    // using c as a char
    for c in s1.chars() {
        println!("{c}");
    }
    // using b as a u8
    for b in s1.bytes() {
        println!("{b}");
    }
    println!();
}
