# aem.py
import hashlib
import os
from cryptography.hazmat.primitives import padding
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend

class AEM:
    def __init__(self, key: bytes):
        self.key = key
        self.backend = default_backend()

    def encrypt(self, plaintext: bytes) -> bytes:
        cipher = Cipher(algorithms.AES(self.key), modes.GCM(iv=os.urandom(12)), backend=self.backend)
        encryptor = cipher.encryptor()
        padder = padding.PKCS7(128).padder()
        padded_data = padder.update(plaintext) + padder.finalize()
        return encryptor.update(padded_data) + encryptor.finalize()

    def decrypt(self, ciphertext: bytes) -> bytes:
        cipher = Cipher(algorithms.AES(self.key), modes.GCM(iv=ciphertext[:12]), backend=self.backend)
        decryptor = cipher.decryptor()
        decrypted_padded_data = decryptor.update(ciphertext[12:]) + decryptor.finalize()
        unpadder = padding.PKCS7(128).unpadder()
        return unpadder.update(decrypted_padded_data) + unpadder.finalize()
