fn use_slices(slices: &mut [i32]) {
    println!("first elem = {}, len = {}", slices[0], slices.len());
    slices[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slices(&mut data[1..4]);
    use_slices(&mut data);
    println!("{:?}", data);
}

fn main() {
    slices();
}
