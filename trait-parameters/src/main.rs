use std::fmt::Debug;

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Square {
    side: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

//fn print_info(shape: impl Shape + Debug) { // normal
// fn print_info<T: Shape + Debug>(shape: T) { // trait bound
fn print_info<T>(shape: T)
where
    T: Shape + Debug, // where clause
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn trait_parameters() {
    let c = Circle { radius: 2.0 };
    print_info(c);
}

fn main() {
    trait_parameters();
}
