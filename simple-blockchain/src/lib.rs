use ring::digest;
use ring::digest::SHA256;

struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        let genesis_block = Self::genesis_block();
        chain.push(genesis_block);

        Self { chain }
    }

    fn genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        }
    }

    fn calculate_hash(data: &[u8]) -> String {
        let hasher = digest::digest(&SHA256, data);
        hex::encode(hasher.as_ref().to_vec())
    }
}
