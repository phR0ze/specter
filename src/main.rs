use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}!", args.name);
}
