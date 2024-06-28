use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::blockchain::Blockchain;

#[derive(Serialize, Deserialize)]
struct BlockchainStorage {
    blockchain: Blockchain,
}

pub struct Storage {
    path: String,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        Storage { path: path.to_string() }
    }

    pub fn save_blockchain(&self, blockchain: &Blockchain) -> Result<(), String> {
        let storage = BlockchainStorage { blockchain: blockchain.clone() };
        let serialized = serde_json::to_string(&storage)?;
        fs::write(self.path.clone(), serialized)?;
        Ok(())
    }

    pub fn load_blockchain(&self) -> Result<Blockchain, String> {
        let contents = fs::read_to_string(self.path.clone())?;
        let storage: BlockchainStorage = serde_json::from_str(&contents)?;
        Ok(storage.blockchain)
    }
}
