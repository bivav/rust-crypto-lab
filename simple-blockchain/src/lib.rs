use ring::digest;
use ring::digest::SHA256;

struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

struct Block {
    index: u32,
    timestamp: u64,
    data: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    merkle_root: String,
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
        let transactions = vec![];

        Block {
            index: 0,
            timestamp: 0,
            data: transactions.clone(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            merkle_root: Self::calculate_merkel_root(&transactions),
            nonce: 0,
        }
    }

    fn calculate_hash(data: &[u8]) -> String {
        let hasher = digest::digest(&SHA256, data);
        hex::encode(hasher.as_ref().to_vec())
    }

    fn calculate_merkel_root(transaction: &[Transaction]) -> String {
        if transaction.is_empty() {
            "0".to_string()
        }

        //TODO: Implement merkle root calculation

        "0".to_string() // dummy value
    }
}
