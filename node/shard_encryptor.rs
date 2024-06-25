// shard_encryptor.rs
use aes::{Aes256, BlockCipher};
use block_modes::{BlockMode, Cbc};
use hex;

struct ShardEncryptor {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl ShardEncryptor {
    fn new(key: Vec<u8>, iv: Vec<u8>) -> Self {
        ShardEncryptor { key, iv }
    }

    fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        // Advanced shard data encryption using AES-256-CBC
        let cipher = Aes256::new_var(key, iv).unwrap();
        let encrypted_data = cipher.encrypt_vec(data).unwrap();
        hex::encode(encrypted_data)
    }

    fn decrypt(&self, encrypted_data: Vec<u8>) -> Vec<u8> {
        // Advanced shard data decryption using AES-256-CBC
        let cipher = Aes256::new_var(key, iv).unwrap();
        let decrypted_data = cipher.decrypt_vec(hex::decode(encrypted_data).unwrap()).unwrap();
        decrypted_data
    }
}
