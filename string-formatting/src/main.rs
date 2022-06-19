fn stirng_formatting() {
    let name = "Suizhe";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}",
        first = "Suizhe",
        last = "Li"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data} {}",
        "alpha",
        "beta",
        "gamma",
        data = "delta"
    );
    println!("{}", mixed);
}

fn main() {
    stirng_formatting();
}
