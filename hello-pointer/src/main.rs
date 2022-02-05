use std::{rc::Rc, fmt::Debug};

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    println!("{:?}", b);
    
    {
        let c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Count after creating a: {}", Rc::strong_count(&a));
        println!("{:?}", c);
    }
    
    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
}
