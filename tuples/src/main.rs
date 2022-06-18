fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    println!("c = {}, d = {}, e = {}, f = {}", c, d, e, f);

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);
}

fn main() {
    tuples();
}
