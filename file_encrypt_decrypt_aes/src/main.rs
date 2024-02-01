use std::env;
use std::error::Error;

use file_encrypt_decrypt_aes::{Config, FileEncryptDecrypt};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args)?;

    if config.command == "encrypt" {
        let mut file_content_buffer = Config::read_file(config)?;
        let (iv, cipher_text) = FileEncryptDecrypt::encrypt(&mut file_content_buffer)?;

        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&iv);
        encrypted_data.extend_from_slice(&cipher_text);

        Config::save_as_base64_encoded_file(encrypted_data, "file.txt")?;
    } else if config.command == "decrypt" {
        let mut file_content_as_buffer = Config::read_file_base64(config)?;
        let decrypted_text = FileEncryptDecrypt::decrypt(&mut file_content_as_buffer, [1, 2, 3])?;

        Config::save_file(decrypted_text, "decrypted.txt")?;
    } else {
        return Err("Invalid command. Usage: cargo run -- [command] [file_path]".into());
    }

    Ok(())
}
