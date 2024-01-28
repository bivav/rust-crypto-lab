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