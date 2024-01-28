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
