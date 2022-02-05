use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct SuparPancakes;

fn main() {
    Pancakes::hello_macro();
    SuparPancakes::hello_macro();
}
