use std::io::stdin;
use std::process::exit;

pub fn get_text_and_keyword() -> (String, String) {
    let mut text = String::new();

    println!("Input your text:");
    if stdin().read_line(&mut text).is_err() {
        println!("Error reading text");
        exit(0);
    }
    let text = text.trim().to_string();


    println!("Input the shift keyword:");

    let mut keyword = String::new();

    if stdin().read_line(&mut keyword).is_err() {
        println!("Error reading key");
        exit(0);
    }

    let keyword = keyword.trim().to_string().to_ascii_lowercase();

    (text, keyword)
}

pub struct VigenereCipher {
    pub text: String,
    pub keyword: String,
}

impl VigenereCipher {
    pub fn new(text: String, keyword: String) -> Self {
        Self {
            text,
            keyword,
        }
    }

    fn get_truncated_keyword(text: &String, keyword: &String) -> String {
        let text_len = text.len();
        let key_len = keyword.len();

        let truncated_keyword = if key_len < text_len {
            let repeat_count = (text_len as f32 / key_len as f32).ceil() as usize;
            let mut trunc_keyword = keyword.repeat(repeat_count);
            trunc_keyword.truncate(text_len);

            trunc_keyword
        } else {
            let mut trunc_keyword = keyword.clone();
            trunc_keyword.truncate(text_len);
            trunc_keyword
        };

        println!("{}", truncated_keyword);

        truncated_keyword
    }

    // pub fn encrypt(&mut self) -> String {
    //     let truncated_keyword = Self::get_truncated_keyword(&self.text, &self.keyword);
    //
    //     self.text.chars().enumerate().map(|(index, c)| {
    //         if c.is_ascii_alphabetic() {
    //             let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
    //             let offset = c as u8 - base;
    //             // let keyword_offset = truncated_keyword.chars().nth(index).unwrap_or('a') as u8 - base;
    //
    //             let keyword_char = truncated_keyword.chars().nth(index).unwrap_or('a') as u8;
    //             let keyword_offset = keyword_char.wrapping_sub(base);
    //
    //
    //             let new_offset = (offset.wrapping_add(keyword_offset)) % 26;
    //
    //             (base + new_offset) as char
    //         } else {
    //             c
    //         }
    //     }).collect()
    // }

    pub fn encrypt(&mut self) -> String {
        let truncated_keyword = Self::get_truncated_keyword(&self.text, &self.keyword);

        self.text.chars().enumerate().map(|(index, c)| {
            if c.is_ascii_alphabetic() {
                // Normalize base to 'a' for lowercase calculation
                let base = 'a' as u8;

                // Normalize text character to lowercase for offset calculation
                let offset = c.to_ascii_lowercase() as u8 - base;

                // Get the corresponding keyword character and normalize to lowercase
                let keyword_char = truncated_keyword.chars().nth(index).unwrap_or('a').to_ascii_lowercase() as u8;

                // Calculate keyword offset from 'a'
                let keyword_offset = keyword_char.wrapping_sub(base);

                // Calculate new offset, ensuring result stays within 0-25 range
                let new_offset = (offset.wrapping_add(keyword_offset)) % 26;

                // Determine the case of the original character to maintain case in the encrypted text
                let encrypted_char = if c.is_ascii_lowercase() {
                    (base + new_offset) as char
                } else {
                    // Convert back to uppercase if original character was uppercase
                    ((base + new_offset) as char).to_ascii_uppercase()
                };

                encrypted_char
            } else {
                c
            }
        }).collect()
    }


    // pub fn decrypt(&mut self) -> String {
    //     let truncated_keyword = Self::get_truncated_keyword(&self.text, &self.keyword);
    //
    //     self.text.chars().enumerate().map(|(index, c)| {
    //         if c.is_ascii_alphabetic() {
    //             let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
    //             let offset = c as u8 - base;
    //             let keyword_char = truncated_keyword.chars().nth(index).unwrap_or('a') as u8;
    //             let keyword_offset = keyword_char.wrapping_sub(base);
    //             // To handle - attempt to add with overflow
    //             let new_offset = ((offset as i16 - keyword_offset as i16 + 26) % 26) as u8;
    //
    //             (base + new_offset) as char
    //         } else {
    //             c
    //         }
    //     }).collect()
    // }

    pub fn decrypt(&mut self) -> String {
        let truncated_keyword = Self::get_truncated_keyword(&self.text, &self.keyword);

        self.text.chars().enumerate().map(|(index, c)| {
            if c.is_ascii_alphabetic() {
                // Normalize base to 'a' for lowercase calculation
                let base = 'a' as u8;

                // Normalize text character to lowercase for offset calculation
                let offset = c.to_ascii_lowercase() as u8 - base;

                // Get the corresponding keyword character and normalize to lowercase
                let keyword_char = truncated_keyword.chars().nth(index).unwrap_or('a').to_ascii_lowercase() as u8;

                // Calculate keyword offset from 'a'
                let keyword_offset = keyword_char - base;

                // Calculate new offset, ensuring result stays within 0-25 range
                let new_offset = ((offset as i16 - keyword_offset as i16 + 26) % 26) as u8;

                // Determine the case of the original character to maintain case in the decrypted text
                let decrypted_char = if c.is_ascii_lowercase() {
                    (base + new_offset) as char
                } else {
                    // Convert back to uppercase if original character was uppercase
                    ((base + new_offset) as char).to_ascii_uppercase()
                };

                decrypted_char
            } else {
                // Non-alphabetic characters are unchanged
                c
            }
        }).collect()
    }

}