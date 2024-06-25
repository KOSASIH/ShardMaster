// storage/blockchain_storage.cpp
#include <iostream>
#include <string>
#include <vector>
#include <openssl/sha.h>

class BlockchainStorage {
public:
    std::vector<std::string> blockchain;

    void addBlock(const std::string& data) {
        std::string blockHash = calculateHash(data);
        blockchain.push_back(blockHash);
    }

    std::string calculateHash(const std::string& data) {
        unsigned char hash[SHA256_DIGEST_LENGTH];
        SHA256_CTX sha256;
        SHA256_Init(&sha256);
        SHA256_Update(&sha256, data.c_str(), data.size());
        SHA256_Final(hash, &sha256);
        std::string blockHash;
        for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
            blockHash += std::to_string(hash[i]);
        }
        return blockHash;
    }

    bool verifyIntegrity() {
        for (int i = 1; i < blockchain.size(); i++) {
            std::string prevBlockHash = blockchain[i - 1];
            std::string currBlockHash = blockchain[i];
            if (currBlockHash!= calculateHash(prevBlockHash)) {
                return false;
            }
        }
        return true;
    }
};
