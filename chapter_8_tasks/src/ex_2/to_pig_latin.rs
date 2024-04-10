pub fn to_pig_latin(string: &str) -> String {
    if string.is_empty() {
        return "".to_string();
    }

    let s = string.to_lowercase();
    let first_char = s.chars().next().unwrap();

    if "aeiou".contains(first_char) {
        format!("{s}-hay")
    } else {
        // let slice = &s[1..]; // Bad practice
        let rest = s.chars().skip(1).collect::<String>();
        format!("{rest}-{first_char}ay")
    }
}

// pub fn to_pig_latin(string: &str) -> None {
//     // чем отличается эта строка
//     let s = &string.to_lowercase();
//     // от этой строки?
//     let s2 = string.to_lowercase();
// }


fn main() {
    let s1 = String::from("first");
    println!("Mutated string: {}", to_pig_latin(&s1));
}