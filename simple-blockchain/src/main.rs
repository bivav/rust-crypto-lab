use anyhow::Result;
use simple_blockchain::Blockchain;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Block chain technology is the future");

    let block = Blockchain::new();
    println!("{:?}", block);

    Ok(())
}
