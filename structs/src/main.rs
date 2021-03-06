#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {}", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let line = Line { start: p, end: p2 };
    println!("line: {:?}", line);
}

fn main() {
    structures();
}
