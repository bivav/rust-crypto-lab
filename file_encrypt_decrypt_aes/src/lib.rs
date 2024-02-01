use std::cmp::min;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::num::NonZeroU32;
use std::process::exit;

use base64::Engine;
use ring::{aead, digest, pbkdf2};
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

    pub fn save_as_base64_encoded_file(data: Vec<u8>, filename: &str) -> Result<bool, Box<dyn Error>> {
        let mut file = File::create(filename)?;
        let encoded_data = base64::engine::general_purpose::STANDARD.encode(&data);
        file.write_all(encoded_data.as_bytes())?;
        Ok(true)
    }


    pub fn save_file(data: String, filename: &str) -> Result<bool, Box<dyn Error>> {
        let mut file = File::create(filename)?;
        file.write(&data.into_bytes())?;
        Ok(true)
    }
}


pub struct FileEncryptDecrypt {}

impl FileEncryptDecrypt {
    pub fn encrypt(file_content: &mut Vec<u8>, password: String) -> Result<([u8; 12], &mut Vec<u8>, [u8; 32]), Box<dyn Error>> {
        let rng = SystemRandom::new(); // Random Number Generator

        // let mut encryption_key = [0u8; 32]; // Creating list of 256 bits of 0s (Encryption Key)
        // rng.fill(&mut encryption_key).unwrap(); // Generating Encryption key

        let mut salt = [0u8; 32]; // Creating list of 256 bits of 0s (Salt)
        rng.fill(&mut salt).unwrap(); // Generating salt

        let mut iv = [0u8; 12]; // Initialization Vector
        rng.fill(&mut iv).unwrap(); // Generating unique IV

        let nonce = Nonce::assume_unique_for_key(iv);

        let password_as_bytes = password.into_bytes();

        let mut encryption_key = [0u8; digest::SHA256_OUTPUT_LEN];

        println!("Generated salt snippet: {:?}", &salt[..6]);
        println!("Generated IV snippet: {:?}", iv);

        let non_zero_iterations = NonZeroU32::new(100_000).unwrap();
        pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, non_zero_iterations, &salt, &password_as_bytes, &mut encryption_key);
        println!("Derived encryption key snippet: {:?}", &encryption_key[..min(encryption_key.len(), 4)]);


        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &encryption_key)
            .map_err(|e| format!("Failed to create unbound key {}", e))?;

        let aead_key = LessSafeKey::new(unbound_key);

        aead_key.seal_in_place_append_tag(nonce, Aad::from(&[]), file_content)
            .map_err(|e| format!("Encryption failed: {:?}", e))?;

        println!("Final encrypted data length: {}", file_content.len());

        Ok((iv, file_content, salt))
    }

    pub fn decrypt(file_content: &mut Vec<u8>, key: &[u8]) -> Result<String, Box<dyn Error>> {
        println!("File content length before decrypting: {}", file_content.len());

        let salt = &file_content[..32];
        let iv = &file_content[32..44];
        println!("Extracted Salt snippet: {:?}", &salt[..6]);
        println!("Extracted IV snipped: {:?}", &iv);

        let mut encryption_key = [0u8; digest::SHA256_OUTPUT_LEN];

        let non_zero_iterations = NonZeroU32::new(100_000).unwrap();

        pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, non_zero_iterations, &salt, &key, &mut encryption_key);

        println!("Encryption key snippet: {:?}", &encryption_key[..min(encryption_key.len(), 4)]);

        let nonce = Nonce::assume_unique_for_key(iv.try_into()?);

        let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &encryption_key)
            .map_err(|e| format!("Failed to create unbound key: {:?}", e))?;

        let aead_key = LessSafeKey::new(unbound_key);


        println!("Data to decrypt length: {}", file_content[44..].len());
        println!("Data to decrypt snippet: {:?}", &file_content[44..50]);


        let decrypted_data = aead_key.open_in_place(nonce, Aad::empty(), &mut file_content[44..])
            .map_err(|_| "Decryption failed: Issue with the key".to_string())?;


        let result = String::from_utf8(decrypted_data.to_vec())
            .map_err(|e| format!("UTF-8 conversion failed: {:?}", e))?;

        Ok(result)
    }
}
