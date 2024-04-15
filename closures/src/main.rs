use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // This closure captures the list variable by reference
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list2 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    
    list2.sort_by_key(|r| r.width);
    println!("{:#?}", list2);
}
