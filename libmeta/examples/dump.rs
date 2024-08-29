use std::{env, fs::File, path::Path};

use libmeta::prelude::*;

fn main() -> anyhow::Result<()> {
    env::set_var("RUST_BACKTRACE", "0"); // Disable backtrace

    // Grab the first argument as the file to read
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    // Open the file
    let f = File::open(Path::new(args[1].as_str()))?;
    jpeg::dump(f)?;

    Ok(())
}
