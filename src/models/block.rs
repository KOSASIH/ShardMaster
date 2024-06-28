use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlockHeader {
    pub number: u32,
    pub previous_block_hash: String,
    pub timestamp: u64,
    pub nonce: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionInput {
    pub transaction_hash: String,
    pub output_index: u32,
    pub script: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionOutput {
    pub value: u64,
    pub script: String,
}
