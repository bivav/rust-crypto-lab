use std::io::stdin;

use anyhow::Result;
use rand::rngs::OsRng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    let start = time::Instant::now();

    let mut rng = OsRng;
    let bits = 2048;

    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    println!("Generated keys {}", start.elapsed().as_secs_f64());

    println!("Enter your message:");
    let mut message = String::new();
    stdin().read_line(&mut message)?;
    message = message.trim().to_string();

    let data = message.as_bytes();
    let encrypt_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data)?;
    assert_ne!(
        &data, &encrypt_data,
        "Data and Encrypted data should not be equal"
    );

    // println!("message : {}\n{:?}", &message, &encrypt_data);

    let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, &encrypt_data)?;
    assert_eq!(&data, &decrypted_data);

    println!("Decrypted Data: {:?}", &decrypted_data);

    Ok(())
}
