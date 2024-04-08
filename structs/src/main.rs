struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    color: Color,
    point: Point,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        color: Color(0, 0, 0),
        point: Point(0, 0, 0),
    }
}

fn main() {
    println!("Hello, world!");
    let username = String::from("user1");
    let email = String::from("user@mail.com");
    let user1 = build_user(email, username);

    // let user2 = User {
    //     email: String::from("mail2@ma.co"),
    //     ..user1
    // };

    // color
    println!("User1: {}", user1.username);
    println!("User1: {}", user1.email);
    println!("User1: {}", user1.sign_in_count);
    println!("User1: {}", user1.active);
    println!("User1: {}", user1.color.0);
    println!("User1: {}", user1.color.1);
    println!("User1: {}", user1.color.2);
    println!("User1: {}", user1.point.0);
    println!("User1: {}", user1.point.1);
    println!("User1: {}", user1.point.2);
}
