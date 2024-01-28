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

    let keyword = keyword.trim().to_string();

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

    pub fn encrypt(&mut self) -> String {
        let text_len = self.text.len();
        let key_len = self.keyword.len();

        let truncated_keyword = if key_len < text_len {
            let repeat_count = (text_len as f32 / key_len as f32).ceil() as usize;

            let mut trunc_keyword = self.keyword.repeat(repeat_count);
            trunc_keyword.truncate(text_len);

            trunc_keyword
        } else {
            let mut trunc_keyword = self.keyword.clone();
            trunc_keyword.truncate(text_len);

            trunc_keyword
        };

        self.text.chars().enumerate().map(|(index, c)| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
                let offset = c as u8 - base;
                let keyword_offset = truncated_keyword.chars().nth(index).unwrap() as u8 - base;

                let new_offset = (offset + keyword_offset) % 26;

                (base + new_offset) as char
            } else {
                c
            }
        }).collect()
    }
}