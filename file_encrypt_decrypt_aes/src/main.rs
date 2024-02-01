use std::env;
use std::error::Error;
use std::io::stdin;
use std::process::exit;

use file_encrypt_decrypt_aes::{Config, FileEncryptDecrypt};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args)?;

    if config.command == "encrypt" {
        let mut file_content_buffer = Config::read_file(config)?;

        let mut password = String::new();

        println!("Enter a password to encrypt the file:");

        stdin().read_line(&mut password).unwrap_or_else(|err| {
            println!("Input proper password: {}", err);
            exit(1);
        });

        let password = password.trim().to_string();

        let (iv, cipher_text, salt) = FileEncryptDecrypt::encrypt(&mut file_content_buffer, password)?;

        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&salt);
        encrypted_data.extend_from_slice(&iv);
        encrypted_data.extend_from_slice(&cipher_text);

        println!("File content length after encrypting: {}", encrypted_data.len());

        let saved = Config::save_as_base64_encoded_file(encrypted_data, "encrypted.txt")?;

        if saved {
            println!("File encrypted as encrypted.txt");
        }


    } else if config.command == "decrypt" {
        let password = rpassword::prompt_password("Enter your password: ").unwrap();

        println!("{:?}", password.to_string());

        if password.is_empty() {
            return Err("Invalid Password!".into());
        }

        let mut file_content_as_buffer = Config::read_file_base64(config)?;
        let decrypted_text = FileEncryptDecrypt::decrypt(&mut file_content_as_buffer, password.trim().as_bytes())?;

        let saved = Config::save_file(decrypted_text, "decrypted.txt")?;

        if saved {
            println!("File decrypted as decrypted.txt");
        }

    } else {
        return Err("Invalid command. Usage: cargo run -- [command] [file_path]".into());
    }

    Ok(())
}
