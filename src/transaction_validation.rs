use crate::transaction::{Transaction, TransactionInput, TransactionOutput};
use crate::block::Block;
use crate::blockchain::Blockchain;

pub fn validate_transaction(transaction: &Transaction, blockchain: &Blockchain) -> bool {
    // Check transaction inputs
    for input in &transaction.inputs {
        if!validate_transaction_input(input, blockchain) {
            return false;
        }
    }

    // Check transaction outputs
    for output in &transaction.outputs {
        if!validate_transaction_output(output) {
            return false;
        }
    }

    // Check transaction hash
    let expected_hash = calculate_transaction_hash(transaction);
    if expected_hash != transaction.hash {
        return false;
    }

    // Check transaction signature
    if!verify_transaction_signature(transaction) {
        return false;
    }

    true
}

fn validate_transaction_input(input: &TransactionInput, blockchain: &Blockchain) -> bool {
    // Check if the input is spending a valid output
    let output = blockchain.get_output(&input.transaction_hash, input.output_index);
    if output.is_none() {
        return false;
    }

    // Check if the input is spending an output that has not been spent before
    if blockchain.is_output_spent(&input.transaction_hash, input.output_index) {
        return false;
    }

    true
}

fn validate_transaction_output(output: &TransactionOutput) -> bool {
    // Check if the output value is valid
    if output.value <= 0 {
        return false;
    }

    true
}

fn calculate_transaction_hash(transaction: &Transaction) -> String {
    let mut hasher = Sha256::new();
    for input in &transaction.inputs {
        hasher.update(input.transaction_hash.as_bytes());
        hasher.update(input.output_index.to_be_bytes());
    }
    for output in &transaction.outputs {
        hasher.update(output.value.to_be_bytes());
    }
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn verify_transaction_signature(transaction: &Transaction) -> bool {
    // TO DO: implement transaction signature verification
    // For now, we'll assume the signature is valid
    true
}
