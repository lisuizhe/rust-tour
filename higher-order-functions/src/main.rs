fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn geater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}

fn higher_order_functions() {
    let limit = 500;
    let mut sum = 0;

    let above_limit = geater_than(limit);

    for i in 0.. {
        let isq = i * i;
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("sum: {}", sum);

    #[allow(clippy::unnecessary_fold)]
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);
}

fn main() {
    higher_order_functions();
}
