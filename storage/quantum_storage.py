# storage/quantum_storage.py
import os
import hashlib
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.backends import default_backend

class QuantumStorage:
    def __init__(self, private_key_path, public_key_path):
        self.private_key = serialization.load_pem_private_key(
            open(private_key_path, 'rb').read(),
            password=None,
            backend=default_backend()
        )
        self.public_key = serialization.load_pem_public_key(
            open(public_key_path, 'rb').read(),
            backend=default_backend()
        )

    def store_data(self, data):
        encrypted_data = self.public_key.encrypt(
            data,
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA256()),
                algorithm=hashes.SHA256(),
                label=None
            )
        )
        return encrypted_data

    def retrieve_data(self, encrypted_data):
        decrypted_data = self.private_key.decrypt(
            encrypted_data,
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA256()),
                algorithm=hashes.SHA256(),
                label=None
            )
        )
        return decrypted_data
