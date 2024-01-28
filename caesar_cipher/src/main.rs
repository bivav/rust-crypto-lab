use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;

use crate::caesar_cipher::CaesarCipher;
use crate::utils::get_text_key;

mod caesar_cipher;
mod utils;

fn main() {
    loop {
        println!("Choices:");
        println!("1. Encrypt\n2. Decrypt\n3.Quit");

        let mut choice = String::new();

        if stdin().read_line(&mut choice).is_err() {
            eprintln!("Error choosing choice. Please select 1 or 2.");
            return;
        }

        let choice: u8 = match choice.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input. Please enter a numeric value.");
                return;
            }
        };

        if choice == 1 {
            let (text, key) = get_text_key();

            let cipher_data = CaesarCipher::new(text, key);
            let encrypted = cipher_data.encrypt();

            println!("Encrypted text: {:?}\n", encrypted);

            sleep(Duration::from_secs(2));
        } else if choice == 2 {

            let (text, key) = get_text_key();

            let cipher_data = CaesarCipher::new(text, key);
            let decrypted = cipher_data.decrypt();

            println!("Encrypted text: {:?}\n", decrypted);

            sleep(Duration::from_secs(2));

        } else {
            println!("Bye bye!");
            return;
        }
    }
}
