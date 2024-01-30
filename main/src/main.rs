use caesar_cipher;
use file_encrypt_decrypt_aes;
use vigenere_cipher;

fn main() {
    loop {
        println!("1. Caesar Cipher\n2. Vigenere Cipher\n3. File Encryption & Decryption\n4. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => (),
            2 => (),
            3 => (),
            4 => break,
            _ => continue,
        }
    }
}
