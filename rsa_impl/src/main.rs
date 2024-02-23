use anyhow::{Context, Result};

mod rsa_from_scratch;
mod rsa_using_lib;

fn main() -> Result<()> {
    println!("Select the program to run:");
    println!("1. RSA from scratch\n2. RSA using rsa library");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).context("Invalid input")?;
    let choice: u32 = choice.trim().parse().context("Invalid input")?;

    match choice {
        1 => rsa_from_scratch::rsa_from_scratch(),
        2 => rsa_using_lib::run_rsa().context("Error running RSA using library")?,
        _ => println!("Invalid choice"),
    }

    Ok(())
}
