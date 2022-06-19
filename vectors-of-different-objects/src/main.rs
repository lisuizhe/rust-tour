#![allow(warnings)]

trait Animal {
    // fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    // fn create(name: &'static str) -> Human {
    //     Human { name }
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name())
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    // fn create(name: &'static str) -> Cat {
    //     Cat { name }
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name())
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn vectors_of_different_objects() {
    let mut creatures = Vec::new();
    creatures.push(Creature::Human(Human { name: "Suizhe" }));
    creatures.push(Creature::Cat(Cat { name: "Fluffy" }));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "Suizhe" }));
    animals.push(Box::new(Cat { name: "Fluffy" }));

    for a in animals.iter() {
        a.talk();
    }
}

fn main() {
    vectors_of_different_objects();
}
