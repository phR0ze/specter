use std::{
    env,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use libmeta::prelude::*;

fn main() -> anyhow::Result<()> {
    env::set_var("RUST_BACKTRACE", "0"); // Disable backtrace

    // 1. Grab the first argument as the file to read
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    // 2. Open the file for reading
    let f = File::open(Path::new(args[1].as_str()))?;

    // 3. Parse the file and pretty print the output
    println!("{}", libmeta::parse(BufReader::new(f))?);
    // let jpeg = Jpeg::parse(BufReader::new(f))?;
    //let mut out = File::create("output.txt")?;
    //write!(out, "{}", jpeg)?;
    //println!("{}", Jpeg::parse(BufReader::new(f))?);

    Ok(())
}
