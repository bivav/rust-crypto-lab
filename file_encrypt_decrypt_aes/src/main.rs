use std::env;
use std::process::exit;

use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};

use file_encrypt_decrypt_aes::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    let mut file_content_buffer = run(config).unwrap_or_else(|err| {
        println!("Application error: {err}");
        exit(1);
    });

    println!("Original: {}\n{:?}\n", file_content_buffer.len(), &file_content_buffer);

    let rng = SystemRandom::new(); // Random Number Generator
    let mut rand_bytes = [0u8; 32]; // Creating list of 256 bits of 0s (Encryption Key)
    rng.fill(&mut rand_bytes).unwrap(); // Replacing the list using RNG

    let mut iv = [0u8; 12]; // Initialization Vector
    rng.fill(&mut iv).unwrap(); // Replacing the list using RNG

    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &rand_bytes)
        .unwrap_or_else(|err| {
            println!("Error creating UnboundKey: {:?}", err);
            exit(1);
        });

    let aead_key = aead::LessSafeKey::new(unbound_key);

    let nonce = aead::Nonce::assume_unique_for_key(iv);

    aead_key.seal_in_place_append_tag(nonce, aead::Aad::from(&[]), &mut file_content_buffer)
        .unwrap_or_else(|err| {
            println!("Error Encrypting the text content: {:?}", err);
        });

    println!("Encrypted: {}\n{:?}\nCiphertext (hex): {:?}", file_content_buffer.len(), &file_content_buffer, hex::encode(&file_content_buffer));
}

