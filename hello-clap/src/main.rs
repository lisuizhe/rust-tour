use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arg {
    /// Name of person to greet
    #[clap(short, long)]
    name: String,

    /// Number of time to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let arg = Arg::parse();
    for _ in 0..arg.count {
        println!("Hello {}!", arg.name);
    }
}
