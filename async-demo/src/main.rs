#![allow(unused)]

use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, World!");
}

struct Song {}

async fn learn_song() -> Song {
    println!("Learn song");
    Song {}
}

async fn sing_song(song: Song) {
    println!("Sing song");
}

async fn dance() {
    println!("Dance");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    let future = hello_world();
    block_on(future);

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main());
}
