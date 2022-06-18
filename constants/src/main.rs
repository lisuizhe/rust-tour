const MEANING_OF_LIFE: u8 = 100; // no fixed address

static mut Z: i32 = 123; // better not use static, use const

fn main() {
    println!("MEANING_OF_LIFE: {}", MEANING_OF_LIFE);

    unsafe {
        println!("z: {}", Z);
    }
}
