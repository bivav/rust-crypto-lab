use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;

use crate::caesar_cipher::CaesarCipher;

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
            println!("Input your text:");
            let mut text = String::from("");
            let mut key = String::from("");

            if stdin().read_line(&mut text).is_err() {
                eprintln!("Error reading text");
                return;
            }

            let text: String = text.trim().to_string();

            println!("Input the shift key:");

            if stdin().read_line(&mut key).is_err() {
                eprintln!("Error reading key");
                return;
            }

            let key = match key.trim().parse::<u8>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a numeric value.");
                    return;
                }
            };

            let cipher_data = CaesarCipher::new(text, key);

            let encrypted = cipher_data.encrypt();

            println!("Encrypted text: {:?}\n", encrypted);

            sleep(Duration::from_secs(2));
        } else if choice == 2 {
            println!("Okay. Decrypting...\n");
            sleep(Duration::from_secs(2));
        }else {
            println!("Bye bye!");
            return;
        }
    }
}
