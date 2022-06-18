#![allow(dead_code)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

fn generics() {
    let a = Point { x: 0.0, y: 4.0 };
    let b = Point { x: 1.2, y: 3.4 };
    println!("a = {:?}, b = {:?}", a, b);

    let line = Line { start: a, end: b };
    println!("line: {:?}", line);
}

fn main() {
    generics();
}
