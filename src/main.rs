use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "phR0ze",
    version, // automatically reads the version in Cargo.toml
    about = "Media curation toolkit",
    long_about = None
)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "Run arbitrary command")]
    Test,

    #[command(about = "Target to get", arg_required_else_help = true)]
    Get { target: String },

    #[command(about = "Target to set", arg_required_else_help = true)]
    Set {
        #[arg(help = "Name of the value being set")]
        key: String,

        #[arg(help = "Value to set")]
        value: String,
    },
}

fn get_target(target: String) {
    println!("Getting target: {}", target);
}

fn set_something(key: String, value: String) {
    println!("Setting key: {}, value: {}", key, value,);
}

fn test() {
    exif();
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Get { target } => get_target(target),
        Commands::Set { key, value } => set_something(key, value),
        Commands::Test => test(),
    }
}
