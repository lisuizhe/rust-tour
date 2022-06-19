#![allow(warnings)]

fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
        // x.push(123); // cannot borrow `*x` as mutable, as it is behind a `&` reference
    };

    let v = vec![3, 2, 1];
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    let b = &mut a;
    *b += 2;
    // println!("a = {}", a); // cannot borrow `a` as immutable because it is also borrowed as mutable
    println!("b = {}", b);

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // z.push(5); // annot borrow `z` as mutable because it is also borrowed as immutable
    }
}

fn main() {
    borrowing();
}
