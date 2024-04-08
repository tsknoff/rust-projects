// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let string = String::from("");

    let first_word = first_word(&string);

    println!("The first word is: {}", first_word);
}
