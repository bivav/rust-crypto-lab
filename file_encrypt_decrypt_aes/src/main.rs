use std::{env};
use std::process::exit;

use ring::rand::{SecureRandom, SystemRandom};
use file_encrypt_decrypt_aes::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    if let Err(e) = run(config){
        println!("Application error: {e}");
        exit(1);
    };

    let rng = SystemRandom::new(); // Random Number Generator
    let mut rand_bytes = [0u8; 256]; // Creating list of 256 bytes of 0s (Encryption Key)

    rng.fill(&mut rand_bytes).unwrap(); // Replacing the list using RNG

    println!("Bytes: {:?}", rand_bytes);



}

