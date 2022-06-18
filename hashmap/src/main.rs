use std::collections::HashMap;

fn hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_default();
        *actual = 0;
    }
    println!("{:?}", shapes);
}

fn main() {
    hashmap();
}
