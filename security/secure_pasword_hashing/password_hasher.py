# password_hasher.py
import bcrypt
import secrets

class PasswordHasher:
    def __init__(self, salt_rounds: int = 12):
        self.salt_rounds = salt_rounds

    def hash_password(self, password: str) -> str:
        salt = secrets.token_bytes(16)
        hashed_password = bcrypt.hashpw(password.encode(), bcrypt.gensalt(self.salt_rounds, salt))
        return hashed_password.decode()

    def verify_password(self, password: str, hashed_password: str) -> bool:
        return bcrypt.checkpw(password.encode(), hashed_password.encode())
