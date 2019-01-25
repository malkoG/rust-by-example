use std::fmt::{self, Debug};

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {p1: pp1, p2: pp2} = rect;
    (pp2.x - pp1.x).abs() * (pp2.y - pp1.y).abs()
}

fn square(point: Point, length: f32) -> Rectangle {
    let Point { x: _x, y: _y } = point;
    Rectangle {
        p1: Point {
            x: _x,
            y: _y,
        },
        p2: Point {
            x: _x + length,
            y: _y + length,
        },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: ({} {})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point};
    println!("second point: ({} {})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("{}", rect_area(_rectangle));
    println!("{:?}", square(Point{x: my_x, y: my_y}, 3.0));
}
