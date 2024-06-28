use crate::block::{Block, BlockHeader};
use crate::blockchain::Blockchain;
use rand::Rng;
use sha2::{Sha256, Digest};

pub struct ProofOfWork {
    block: Block,
    target: Vec<u8>,
}

impl ProofOfWork {
    pub fn new(block: Block, blockchain: &Blockchain) -> Self {
        let target = blockchain.get_target();
        ProofOfWork { block, target }
    }

    pub fn run(&mut self) -> bool {
        let mut nonce = 0;
        loop {
            self.block.header.nonce = nonce;
            let hash = self.calculate_hash();
            if self.is_valid_hash(&hash) {
                return true;
            }
            nonce += 1;
        }
    }

    fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.block.header.number.to_be_bytes());
        hasher.update(self.block.header.previous_block_hash.as_bytes());
        hasher.update(self.block.header.timestamp.to_be_bytes());
        hasher.update(self.block.header.nonce.to_be_bytes());
        for transaction in &self.block.transactions {
            hasher.update(transaction.hash.as_bytes());
        }
        let hash = hasher.finalize();
        hash.to_vec()
    }

    fn is_valid_hash(&self, hash: &Vec<u8>) -> bool {
        for i in 0..self.target.len() {
            if hash[i] > self.target[i] {
                return false;
            }
        }
        true
    }
}
