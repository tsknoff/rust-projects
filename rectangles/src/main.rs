#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Using single values
    let width1 = 30;
    let height1 = 50;

    // Using tuples
    let rec = (30, 50);

    let scale = 2;

    // Using struct
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    // Normal print
    println!("rect1 is {:?}", rect1);
    // Pretty print
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_values(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_tuples(rec)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_struct(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("Width is greater than 0: {}", rect1.width);
    } else {
        println!("Width is less than or equal to 0");
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("Square: {:#?}", square);
}

fn area_using_values(width: u32, height: u32) -> u32 {
    width * height
}

fn area_using_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
