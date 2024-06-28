use crate::blockchain::Blockchain;
use crate::pow::ProofOfWork;
use crate::storage::Storage;
use crate::transaction::Transaction;

pub struct BlockchainAPI {
    blockchain: Blockchain,
    storage: Storage,
    pow: ProofOfWork,
}

impl BlockchainAPI {
    pub fn new(storage_path: &str) -> Self {
        let storage = Storage::new(storage_path);
        let blockchain = storage.load_blockchain().unwrap_or_default();
        let pow = ProofOfWork::new(blockchain.clone(), &blockchain);
        BlockchainAPI { blockchain, storage, pow }
    }

    pub fn create_new_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        let new_block = self.blockchain.create_new_block(transactions)?;
        self.pow.block = new_block.clone();
        if self.pow.run() {
            self.blockchain.add_block(new_block)?;
            self.storage.save_blockchain(&self.blockchain)?;
            Ok(())
        } else {
            Err("Failed to mine new block".to_string())
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.blockchain.add_transaction(transaction)?;
        Ok(())
    }

    pub fn get_blockchain_state(&self) -> &Blockchain {
        &self.blockchain
    }
}
