extern crate piston_window;

use piston_window::*;

// let CLIENT_WIDTH: u32 = 800;
// let CLIENT_HEIGHT: u32 = 600;
const CLIENT_WIDTH: u32 = 800;
const CLIENT_HEIGHT: u32 = 600;

const GRID_SIZE: u32 = 8;
const SCALE: u32 = 100;

struct Point {
    x: u32,
    y: u32,
    color: Color,
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [CLIENT_WIDTH, CLIENT_HEIGHT])
            .exit_on_esc(true).build().unwrap();

    let mut points: Vec<Point> = Vec::new();
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            points.push(Point {
                x: x * SCALE,
                y: y * SCALE,
                color: Color {
                    r: rand::random::<f32>(),
                    g: rand::random::<f32>(),
                    b: rand::random::<f32>(),
                    a: 1.0,
                },
            });
        }
    }

    let mut snake = Snake {
        body: vec![
            Point {
                x: 0,
                y: 0,
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            },
        ],
        direction: Direction::Right,
    };


    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            for point in &points {
                rectangle(
                    [point.color.r, point.color.g, point.color.b, point.color.a],
                    [point.x as f64, point.y as f64, SCALE as f64, SCALE as f64],
                    context.transform,
                    graphics,
                );
            }

            for point in &snake.body {
                rectangle(
                    [point.color.r, point.color.g, point.color.b, point.color.a],
                    [point.x as f64, point.y as f64, SCALE as f64, SCALE as f64],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}