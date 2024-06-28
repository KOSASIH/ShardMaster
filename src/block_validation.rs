use crate::block::{Block, BlockHeader};
use crate::transaction::{Transaction, TransactionInput, TransactionOutput};
use sha2::{Sha256, Digest};

pub fn validate_block(block: &Block) -> bool {
    // Check block header
    if!validate_block_header(&block.header) {
        return false;
    }

    // Check transactions
    for transaction in &block.transactions {
        if!validate_transaction(transaction) {
            return false;
        }
    }

    // Check block hash
    let expected_hash = calculate_block_hash(&block);
    if expected_hash!= block.hash {
        return false;
    }

    true
}

fn validate_block_header(header: &BlockHeader) -> bool {
    // Check block number
    if header.number == 0 {
        return false; // Genesis block has a special case
    }

    // Check previous block hash
    if header.previous_block_hash.is_empty() {
        return false;
    }

    // Check timestamp
    if header.timestamp == 0 {
        return false;
    }

    true
}

fn validate_transaction(transaction: &Transaction) -> bool {
    // Check transaction inputs
    for input in &transaction.inputs {
        if!validate_transaction_input(input) {
            return false;
        }
    }

    // Check transaction outputs
    for output in &transaction.outputs {
        if!validate_transaction_output(output) {
            return false;
        }
    }

    true
}

fn validate_transaction_input(input: &TransactionInput) -> bool {
    // Check transaction hash
    if input.transaction_hash.is_empty() {
        return false;
    }

    // Check output index
    if input.output_index == 0 {
        return false;
    }

    true
}

fn validate_transaction_output(output: &TransactionOutput) -> bool {
    // Check value
    if output.value == 0 {
        return false;
    }

    true
}

fn calculate_block_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(block.header.number.to_be_bytes());
    hasher.update(block.header.previous_block_hash.as_bytes());
    hasher.update(block.header.timestamp.to_be_bytes());
    hasher.update(block.header.nonce.to_be_bytes());
    for transaction in &block.transactions {
        hasher.update(transaction.hash.as_bytes());
    }
    let hash = hasher.finalize();
    format!("{:x}", hash)
}
