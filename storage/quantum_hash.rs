// storage/quantum_hash.rs
use sha3::{Sha3_256, Digest};
use rand::Rng;

struct QuantumHashStorage {
    hash_map: std::collections::HashMap<Vec<u8>, Vec<u8>>,
}

impl QuantumHashStorage {
    fn new() -> Self {
        QuantumHashStorage {
            hash_map: std::collections::HashMap::new(),
        }
    }

    fn store_data(&mut self, data: Vec<u8>) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let salt: Vec<u8> = rng.gen_iter::<u8>().take(16).collect();
        let mut hash_input = data.clone();
        hash_input.extend_from_slice(&salt);
        let hash = Sha3_256::digest(&hash_input);
        self.hash_map.insert(hash.to_vec(), data);
        hash.to_vec()
    }

    fn retrieve_data(&self, hash: Vec<u8>) -> Option<Vec<u8>> {
        self.hash_map.get(&hash).cloned()
    }
}
