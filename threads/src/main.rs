#![allow(warnings)]

use std::thread;
use std::time;

fn threads() {
    let handle = thread::spawn(|| {
        print!("+");
        thread::sleep(time::Duration::from_millis(500));
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    handle.join();
}

fn main() {
    threads();
}
