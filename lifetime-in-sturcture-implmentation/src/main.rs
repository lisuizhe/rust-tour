#![allow(warnings)]

struct Person<'a> {
    // name: &str, // missing lifetime specifier
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn lifetime_in_structure_implementation() {
    let person = Person { name: "Suizhe" };
}

fn main() {
    lifetime_in_structure_implementation();
}
