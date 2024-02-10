use std::env;
use std::error::Error;
use std::io::stdin;

use anyhow::{Context, Result};

use file_encrypt_decrypt_aes::{Config, FileEncryptDecrypt};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args)?;

    if config.command == "encrypt" {
        let mut file_content_buffer = Config::read_file(config)?;

        // Hash value before encryption
        let before_encrypt_hash = FileEncryptDecrypt::get_hash(file_content_buffer.as_slice());
        println!(
            "Hash before encryption: {:?}",
            hex::encode(&before_encrypt_hash)
        );

        println!("Enter a password to encrypt the file:");
        let mut password = String::new();

        stdin()
            .read_line(&mut password)
            .context("Input valid password")?;

        let password = password.trim().to_string();

        let (iv, cipher_text, salt) =
            FileEncryptDecrypt::encrypt(&mut file_content_buffer, password)?;

        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&before_encrypt_hash);
        encrypted_data.extend_from_slice(&salt);
        encrypted_data.extend_from_slice(&iv);
        encrypted_data.extend_from_slice(&cipher_text);

        println!(
            "File content length after encrypting: {}",
            encrypted_data.len()
        );

        let after_encrypt_hash = FileEncryptDecrypt::get_hash(encrypted_data.as_slice());
        println!(
            "Hash after encryption: {:?}",
            hex::encode(&after_encrypt_hash)
        );

        let saved = Config::save_as_base64_encoded_file(encrypted_data, "encrypted.txt")?;

        if saved {
            println!("File encrypted as encrypted.txt");
        }
    } else if config.command == "decrypt" {
        let password = rpassword::prompt_password("Enter your password: ").unwrap();

        if password.is_empty() {
            return Err("Invalid Password!".into());
        }

        let mut file_content_as_buffer = Config::read_file_base64(config)?;
        let before_decryption_hash =
            FileEncryptDecrypt::get_hash(file_content_as_buffer.as_slice());
        let decrypted_text =
            FileEncryptDecrypt::decrypt(&mut file_content_as_buffer, password.trim().as_bytes())?;

        println!("Encrypted Hash: {:?}", hex::encode(before_decryption_hash));
        let verify = FileEncryptDecrypt::verify_hash(
            file_content_as_buffer.as_slice(),
            decrypted_text.as_bytes(),
        );

        if verify {
            println!("Hashes match!");
            let saved = Config::save_file(decrypted_text, "decrypted.txt")?;
            if saved {
                println!("File decrypted as decrypted.txt");
            } else {
                println!("Error saving file");
            }
        } else {
            println!("Hashes don't match! File is corrupted!");
        }
    } else {
        eprintln!("Error: Invalid command. Usage: cargo run -- [command] [file_path]");
        return Err("The [command] should be either 'encrypt' or 'decrypt', and [file_path] should be the path to the file you want to encrypt or decrypt".into());
    }

    Ok(())
}
