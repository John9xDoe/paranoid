extern crate aes;
extern crate block_modes;
extern crate rand;
use std::{env, process};

use paranoid::{run, Config};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Parsing arguments error: {}", err);
        process::exit(1)
    });
    
    if let Err(e) = run(config)  {
        eprintln!("Program error: {}", e);
        process::exit(1);
    }
}

