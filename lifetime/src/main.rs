#![allow(warnings)]

struct Person {
    name: String,
}

impl Person {
    // fn get_ref_name(&'a self) -> &'a String {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

struct Company<'p> {
    name: String,
    // ceo: Person, // missing lifetime specifier
    ceo: &'p Person,
}

fn lifetime() {
    let boss = Person {
        name: String::from("Elon Musk"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    let mut z: &String;
    // {
    //     let p = Person {
    //         name: String::from("Suizhe"),
    //     };
    //     z = p.get_ref_name(); // `p` does not live long enough
    // }
    // println!("z: {}", z);
}

fn main() {
    lifetime();
}
