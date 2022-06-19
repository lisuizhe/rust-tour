#![allow(warnings)]

use std::rc::Rc;

// struct Person {
//     name: String,
// }

// impl Person {
//     fn new(name: String) -> Person {
//         Person { name }
//     }

//     fn greet(&self) {
//         println!("Hi, my name is {}", self.name);
//     }
// }

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn reference_counted_variables() {
    // let name = "Suizhe".to_string();
    // let person = Person::new(name);

    // person.greet();
    // println!("Name = {}", name); // borrow of moved value: `name`

    let name = Rc::new("Suizhe".to_string());
    println!(
        "Name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name),
    );
    {
        let person = Person::new(name.clone());
        println!(
            "Name = {}, name has {} strong pointers",
            name,
            Rc::strong_count(&name),
        );
        person.greet();
    }
    println!(
        "Name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name),
    );
}

fn main() {
    reference_counted_variables();
}
