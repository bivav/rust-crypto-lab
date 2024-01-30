use std::error::Error;
use std::fs::File;
use std::io::Read;

pub struct Config {
    pub key: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let key = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { key, file_path })
    }
}

pub fn run(config: Config) -> Result<Vec<u8>, Box<dyn Error>> {
    // let file_content = fs::read_to_string(config.file_path)?;

    let mut file = File::open(config.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!("{:?}", buffer);

    Ok(buffer)


    // println!("{}\n{:?}", config.key, file_content);
    // Ok(())
}