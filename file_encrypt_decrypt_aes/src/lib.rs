use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use base64::Engine;
use ring::aead;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey};
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

    pub fn read_file(config: Config) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(config.file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn read_file_base64(config: Config) -> Result<Vec<u8>, Box<dyn Error>> {
        let file_content = fs::read_to_string(config.file_path)?;

        let decoded_data = base64::engine::general_purpose::STANDARD.decode(&file_content)
            .unwrap_or_else(|e| {
                println!("Decoding Error: {:?}\nAre you sure it's the encrypted file?", e);
                exit(1);
            });

        Ok(decoded_data)
    }

    pub fn save_as_base64_encoded_file(data: Vec<u8>, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        let encoded_data = base64::engine::general_purpose::STANDARD.encode(&data);
        file.write_all(encoded_data.as_bytes())?;
        Ok(())
    }


    pub fn save_file(data: String, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        file.write(&data.into_bytes())?;
        Ok(())
    }
}


pub struct FileEncryptDecrypt {}

impl FileEncryptDecrypt {
    pub fn encrypt(file_content: &mut Vec<u8>) -> Result<([u8; 12], &mut Vec<u8>), Box<dyn Error>> {
        let rng = SystemRandom::new(); // Random Number Generator
        let mut encryption_key = [0u8; 32]; // Creating list of 256 bits of 0s (Encryption Key)
        rng.fill(&mut encryption_key).unwrap(); // Replacing the list using RNG

        let mut iv = [0u8; 12]; // Initialization Vector
        rng.fill(&mut iv).unwrap(); // Replacing the list using RNG
        let nonce = Nonce::assume_unique_for_key(iv);

        // TODO: Important to remove this block of code.
        let mut file = File::create("key.txt").expect("Error");
        file.write_all(&encryption_key).expect("TODO: panic message");

        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &encryption_key)
            .map_err(|e| format!("Failed to create unbound key {}", e))?;

        let aead_key = LessSafeKey::new(unbound_key);

        aead_key.seal_in_place_append_tag(nonce, Aad::from(&[]), file_content)
            .map_err(|e| format!("Encryption failed: {:?}", e))?;

        Ok((iv, file_content))
    }

    pub fn decrypt(file_content: &mut Vec<u8>, _key: [u8; 3]) -> Result<String, Box<dyn Error>> {
        // Read the encryption key from a file.
        // TODO: Important to remove this block of code.
        let key = fs::read("key.txt")?;

        let iv = &file_content[0..12];
        let nonce = Nonce::assume_unique_for_key(iv.try_into()?);

        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &key)
            .map_err(|e| format!("Failed to create unbound key: {:?}", e))?;
        let aead_key = LessSafeKey::new(unbound_key);

        let decrypted_data = aead_key.open_in_place(nonce, Aad::empty(), &mut file_content[12..])
            .map_err(|e| format!("Decryption failed: {:?}", e))?;

        let result = String::from_utf8(decrypted_data.to_vec())
            .map_err(|e| format!("UTF-8 conversion failed: {:?}", e))?;

        Ok(result)
    }
}
