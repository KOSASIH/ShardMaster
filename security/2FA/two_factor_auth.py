# two_factor_auth.py
import pyotp
import qrcode

class TwoFactorAuth:
    def __init__(self, secret_key: str):
        self.secret_key = secret_key

    def generate_qr_code(self, user_id: str) -> bytes:
        totp = pyotp.TOTP(self.secret_key)
        provisioning_uri = totp.provisioning_uri(name=user_id, issuer_name="ShardMaster")
        qr_code = qrcode.make(provisioning_uri)
        return qr_code.tobytes()

    def verify_token(self, token: str) -> bool:
        totp = pyotp.TOTP(self.secret_key)
        return totp.verify(token)
