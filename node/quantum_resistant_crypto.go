// quantum_resistant_crypto.go
package main

import (
	"crypto/rand"
	"crypto/sha256"
	"golang.org/x/crypto/ed25519"
)

type QuantumResistantCrypto struct{}

func (qrc *QuantumResistantCrypto) GenerateKeyPair() ([]byte, []byte, error) {
	// Generate ed25519 key pair
	publicKey, privateKey, err := ed25519.GenerateKey(rand.Reader)
	return publicKey, privateKey, err
}

func (qrc *QuantumResistantCrypto) Encrypt(data []byte, publicKey []byte) ([]byte, error) {
	// Hybrid encryption using ed25519 and AES-256
	aesKey := make([]byte, 32)
	_, err := rand.Read(aesKey)
	if err != nil {
		return nil, err
	}
	ciphertext := make([]byte, len(data))
	aesEncrypter, err := aes.NewCipher(aesKey)
	if err != nil {
		return nil, err
	}
	aesEncrypter.CryptBlocks(ciphertext, data)
	ed25519Signature, err := ed25519.Sign(publicKey, aesKey)
	if err != nil {
		return nil, err
	}
	return append(ciphertext, ed25519Signature...), nil
}

func (qrc *QuantumResistantCrypto) Decrypt(encryptedData []byte, privateKey []byte) ([]byte, error) {
	// Hybrid decryption using ed25519 and AES-256
	ed25519Signature := encryptedData[len(encryptedData)-ed25519.SignatureSize:]
	ciphertext := encryptedData[:len(encryptedData)-ed25519.SignatureSize]
	aesKey, err := ed25519.Verify(privateKey, ed25519Signature, ciphertext)
	if err != nil {
		return nil, err
	}
	aesDecrypter, err := aes.NewCipher(aesKey)
	if err != nil {
		return nil, err
	}
	plaintext := make([]byte, len(ciphertext))
	aesDecrypter.CryptBlocks(plaintext, ciphertext)
	return plaintext, nil
}
