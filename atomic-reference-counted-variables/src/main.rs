#![allow(warnings)]

// use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn atomic_reference_counted_variables() {
    let name = Arc::new("Suizhe".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    }); // `std::rc::Rc<std::string::String>` cannot be sent between threads safely
    println!("Name = {}", name);
}

fn main() {
    atomic_reference_counted_variables();
}
