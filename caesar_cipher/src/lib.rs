use std::io::stdin;
use std::process::exit;

pub fn get_text_key() -> (String, u8) {
    let mut text = String::new();

    println!("Input your text:");
    if stdin().read_line(&mut text).is_err() {
        println!("Error reading text");
        exit(0);
    }
    let text = text.trim().to_string();

    let key: u8;
    loop {
        println!("Input the shift key (number):");
        let mut key_str = String::new();
        if stdin().read_line(&mut key_str).is_err() {
            println!("Error reading key");
            continue;
        }

        match key_str.trim().parse::<u8>() {
            Ok(num) => {
                key = num;
                break;
            }
            Err(_) => {
                println!("Invalid input. Please enter a numeric value.");
                continue;
            }
        };
    }

    (text, key)
}

pub struct CaesarCipher {
    pub text: String,
    pub key: u8,
}

impl CaesarCipher {
    pub fn new(text: String, key: u8) -> Self {
        Self {
            text,
            key,
        }
    }

    pub fn encrypt(&self) -> String {
        self.text.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
                let offset = c as u8 - base;
                let new_offset = (offset + self.key) % 26;
                (base + new_offset) as char
            } else {
                c
            }
        }).collect()
    }

    pub fn decrypt(&self) -> String {
        self.text.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
                let offset = c as u8 - base;
                // To handle - attempt to add with overflow
                let new_offset = ((offset as i16 - self.key as i16 + 26) % 26) as u8;
                (base + new_offset) as char
            } else {
                c
            }
        }).collect()
    }
}