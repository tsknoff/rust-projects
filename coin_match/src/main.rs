#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //Example of match
    println!("#1 Example of match");
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));


    //Example of match with Option
    println!();
    println!("#2 Example of match with Option");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five: {:?}", five);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    //Example of if let
    let config_max = Some(3u8); //3u8 is a way to specify the type of the number
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //Example match coin
    // let coin = Coin::Quarter(UsState::Alaska);
}
