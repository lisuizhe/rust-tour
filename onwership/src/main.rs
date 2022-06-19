#![allow(warnings)]

fn ownership() {
    let v = vec![1, 2, 3];

    // let v2 = v;

    // let foo = |v: Vec<i32>| {};
    // foo(v);

    // println!("v = {:?}", v); // borrow of moved value: `v`, value borrowed here after move

    // let u = Box::new(1);
    let u = 1;
    let u2 = u;

    println!("u = {}", u);

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let vv = print_vector(v);
    println!("vv: {:?}", vv);
}

fn main() {
    ownership();
}
