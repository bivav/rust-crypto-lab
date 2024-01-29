use std::io::stdin;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use ::vigenere_cipher::{get_text_and_keyword, VigenereCipher};

mod vigenere_cipher;

fn main() {
    loop {
        println!("Choices:");
        println!("1. Encrypt\n2. Decrypt\n3.Quit");

        let mut choice = String::new();

        if stdin().read_line(&mut choice).is_err() {
            println!("Error choosing choice. Please select from the Menu\n");
            exit(1);
        }

        let choice: u8 = match choice.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input. Please enter a numeric value.\n");
                continue;
            }
        };


        match choice {
            1 => {
                let (text, keyword) = get_text_and_keyword();
                let mut cipher_data = VigenereCipher::new(text, keyword);

                let encrypted_data = cipher_data.encrypt();

                println!("Encrypted: {}", encrypted_data);

                sleep(Duration::from_secs(2));
            }
            2 => {
                let (text, keyword) = get_text_and_keyword();
                let mut cipher_data = VigenereCipher::new(text, keyword);

                let decrypted_data = cipher_data.decrypt();

                println!("Decrypted: {}", decrypted_data);

                sleep(Duration::from_secs(2));
            }
            3 => {
                exit(0);
            }
            _ => {
                println!("Please choose from the Menu\n");
                sleep(Duration::from_secs(1));
                continue;
            }
        }
    }
}
