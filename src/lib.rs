use std::error::Error;
use rand::random;

pub mod crypt;
use crate::crypt::encrypt_file;
use crate::crypt::decrypt_file;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.algo == "aes256" && config.mode == "encrypt" {
        let key = [0u8; 32]; // random::<[u8; 32]>();
        let iv = random::<[u8; 16]>();

        encrypt_file(&config.dir, &key, &iv)?;
        println!("File is encrypted successfully");
        Ok(())
    } else if config.algo == "aes256" && config.mode == "decrypt" {
        let key = [0u8; 32];
        decrypt_file(&config.dir, &key)?;
        println!("File is decrypted successfully");
        Ok(())
    } else {
        Err("unknown algorithm".into())
    }
}

pub struct Config {
    dir: String,
    algo: String,
    mode: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{

        if args.len() < 3 {
            eprintln!("Not enough arguments")
        }

        let dir = args[1].clone();
        let algo = args[2].clone();
        let mode = args[3].clone();

        Ok(Config { dir, algo, mode })
    }
}