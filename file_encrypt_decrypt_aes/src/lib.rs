use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use base64::Engine;
use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};

pub struct Config {
    pub command: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let command = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { command, file_path })
    }
}

pub fn run(config: Config) -> Result<Vec<u8>, Box<dyn Error>> {
    // let file_content = fs::read_to_string(config.file_path)?;

    let mut file = File::open(config.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn save_file(data: Vec<u8>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    let encoded_data = base64::engine::general_purpose::STANDARD.encode(&data);
    file.write_all(encoded_data.as_bytes())?;

    Ok(())
}

pub struct FileEncryptDecrypt {}

impl FileEncryptDecrypt {
    pub fn encrypt(file_content: &mut Vec<u8>) -> ([u8; 12], &mut Vec<u8>) {
        let rng = SystemRandom::new(); // Random Number Generator
        let mut encryption_key = [0u8; 32]; // Creating list of 256 bits of 0s (Encryption Key)
        rng.fill(&mut encryption_key).unwrap(); // Replacing the list using RNG

        let mut iv = [0u8; 12]; // Initialization Vector
        rng.fill(&mut iv).unwrap(); // Replacing the list using RNG

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &encryption_key)
            .unwrap_or_else(|err| {
                println!("Error creating UnboundKey: {:?}", err);
                exit(1);
            });

        let aead_key = aead::LessSafeKey::new(unbound_key);

        let nonce = aead::Nonce::assume_unique_for_key(iv);

        aead_key.seal_in_place_append_tag(nonce, aead::Aad::from(&[]), file_content)
            .unwrap_or_else(|err| {
                println!("Error Encrypting the text content: {:?}", err);
            });

        (iv, file_content)
    }
}
