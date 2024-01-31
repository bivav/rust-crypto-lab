use std::env;
use std::process::exit;

use file_encrypt_decrypt_aes::{Config, FileEncryptDecrypt, run, save_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    if config.command == "encrypt" {
        let mut file_content_buffer = run(config).unwrap_or_else(|err| {
            println!("Application error: {err}");
            exit(1);
        });

        let (iv, cipher_text) = FileEncryptDecrypt::encrypt(&mut file_content_buffer);

        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&iv);
        encrypted_data.extend_from_slice(&cipher_text);

        if let Err(e) = save_file(encrypted_data.clone(), "file.txt") {
            println!("Saving error: {}", e);
            exit(1);
        }

        println!("Encrypted: {}\n{:?}", cipher_text.len(), &cipher_text);
        println!("Nonce (hex): {:?}\nCiphertext (hex): {:?}", hex::encode(iv), hex::encode(&encrypted_data));
    } else if config.command == "decrypt" {
        println!("Decryption coming soon");
        exit(0);
    } else {
        println!("Error:\nusage: cargo run -- [command] [file_path] ");
        exit(1);
    }
}

