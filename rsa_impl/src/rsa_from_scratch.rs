use std::thread::sleep;
use std::time::Duration;

use glass_pumpkin::prime;
use num_bigint::BigUint;

use rsa_impl::RSAAlgo;

pub fn rsa_from_scratch() {
    println!("Input message to encrypt:");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();

    let message = message.trim();

    println!("Generating two 2048-bit prime numbers...");
    let start_time = std::time::Instant::now();

    const BITS: usize = 2048;

    let p: BigUint = prime::new(BITS).unwrap();
    let q: BigUint = prime::new(BITS).unwrap();

    println!("Prime p: {}", p);
    println!("Prime q: {}", q);
    println!(
        "\nGenerated primes in {} seconds\n",
        start_time.elapsed().as_secs_f64()
    );

    println!("Calculating public and private keys...");
    sleep(Duration::from_secs(3));

    let rsa_algo = RSAAlgo::new(&p, &q);

    // calculate modulus n
    let n = rsa_algo.generate_modulus();

    // calculate Euler's totient
    let phi = rsa_algo.eulers_totient();

    // choose public exponent e as 65537
    let e = RSAAlgo::choose_public_exponent();

    println!("Modulus n length: {}", n.to_bytes_be().len());
    println!("Totient phi length: {}", phi.to_bytes_be().len());
    println!("Public exponent e: {}", e);

    println!("\nCalculating private exponent d...");
    sleep(Duration::from_secs(2));

    // calculate private exponent d
    let d = RSAAlgo::mod_inverse(&e.clone(), &phi.clone()).unwrap();

    println!("Private exponent d length: {:?}\n", d.to_bytes_be().len());

    println!("Encryption and decryption in progress...\n");

    let message_bytes = BigUint::from_bytes_be(message.as_bytes());
    let encrypted = RSAAlgo::encrypt(&message_bytes, &e, &n);
    let decrypted = RSAAlgo::decrypt(&encrypted, &d, &n);

    println!("Encrypted message length: {:?} ", encrypted.to_bytes_be().len());
    println!("Message bytes: {}", message_bytes);
    println!("Decrypted: {}", decrypted);
    println!(
        "Decrypted text: {}\n",
        String::from_utf8_lossy(&decrypted.to_bytes_be().as_slice())
    );

    println!(
        "Total time for RSA encryption and decryption: {:.2} seconds",
        start_time.elapsed().as_secs_f64()
    );

    assert_eq!(
        message_bytes, decrypted,
        "Decryption failed to recover original value"
    );
}
