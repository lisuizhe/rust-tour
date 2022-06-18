#![allow(clippy::vec_init_then_push)]

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    let idx: usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("no such element on a[6]"),
    }

    for x in &a {
        println!("{}", x)
    }

    a.push(77);
    println!("a = {:?}", a);

    let last_elem = a.pop();
    println!("last elem: {:?}, a: {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn main() {
    vectors();
}
