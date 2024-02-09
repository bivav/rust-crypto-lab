use std::io;
use std::io::{stdin, Write};

use anyhow::Result;
use glass_pumpkin::{prime, safe_prime};
use num_bigint::{BigUint, ToBigUint};
use rand::rngs::OsRng;
use tokio::time;

use rsa::RSAAlgo;

#[tokio::main]
async fn main() -> Result<()> {
    let mut rng = OsRng;

    let start = time::Instant::now();

    // println!("Time: {:?}", start.elapsed().as_secs_f64());

    println!("Generating prime numbers...");
    // let p: BigUint = safe_prime::from_rng(128, &mut rng).unwrap();
    // let q: BigUint = safe_prime::from_rng(128, &mut rng).unwrap();

    let p = 61.to_biguint().unwrap();
    let q = 53.to_biguint().unwrap();

    println!("Prime p: {}", p);
    println!("Prime q: {}", q);

    println!("Calculating public and private keys...");
    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);
    let e = RSAAlgo::choose_public_exponent();
    let d = RSAAlgo::mod_inverse(e.clone(), phi.clone());
    println!("Modulus n: {}", n);
    println!("Totient phi: {}", phi);
    println!("Public exponent e: {}", e);
    println!("Private exponent d: {}", d);

    // io::stdout().flush().unwrap();
    //
    // println!("Enter your message:");
    // let mut message = String::new();
    // stdin().read_line(&mut message)?;

    // let message = message.trim().to_string();
    // println!("Original message: {}", message);
    //
    // let message_bytes = message.as_bytes();
    // println!("Original message bytes: {:?}", message_bytes);
    //
    // let message_biguint = BigUint::from_bytes_be(message_bytes);
    // println!("Original message as BigUint: {}", message_biguint);
    //
    // let encrypted = RSAAlgo::encrypt(&message_biguint, &e, &n);
    // println!("Encrypted message: {}", encrypted);
    //
    // let decrypted = RSAAlgo::decrypt(&encrypted, &d, &n);
    // println!("Decrypted BigUint: {}", decrypted);
    //
    // let decrypted_bytes = decrypted.to_bytes_be();
    // println!("Decrypted bytes: {:?}", decrypted_bytes);
    //
    // match String::from_utf8(decrypted_bytes.clone()) {
    //     Ok(decrypted_message) => println!("Decrypted Message: {}", decrypted_message),
    //     Err(e) => {
    //         println!("Failed to convert decrypted bytes to string: {:?}", e);
    //         println!(
    //             "Valid bytes up to: {:?}",
    //             &decrypted_bytes[..e.utf8_error().valid_up_to()]
    //         );
    //         // Optionally, attempt to print the valid part of the message
    //         match String::from_utf8(decrypted_bytes[..e.utf8_error().valid_up_to()].to_vec()) {
    //             Ok(valid_part) => println!("Valid part of the message: {}", valid_part),
    //             Err(_) => println!("Cannot display the valid part of the message."),
    //         }
    //     }
    // }


    // Simple encryption and decryption test with a known BigUint value
    let test_value = BigUint::from(123u32);
    // let encrypted = test_value.modpow(&e, &n);
    let encrypted = RSAAlgo::modular_exponentiation(&test_value, &e, &n);
    // let decrypted = encrypted.modpow(&d, &n);
    let decrypted = RSAAlgo::modular_exponentiation(&encrypted, &d, &n);

    println!("Test value: {}", test_value);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
    assert_eq!(test_value, decrypted, "Decryption failed to recover original value");

    // Convert a simple message to bytes and then to BigUint, and back
//     let message = "Hi";
//     let message_bytes = message.as_bytes();
//     let message_biguint = BigUint::from_bytes_be(message_bytes);
//
// // Convert back from BigUint to bytes and attempt UTF-8 conversion
//     let recovered_bytes = message_biguint.to_bytes_be();
//     let recovered_message = String::from_utf8(recovered_bytes).expect("Failed to convert bytes to string");
//
//     println!("Original message: {}", message);
//     println!("Recovered message: {}", recovered_message);
//     assert_eq!(message, recovered_message, "Message recovery failed");


    println!("Total Time: {:?}", start.elapsed().as_secs_f64());




    Ok(())
}
