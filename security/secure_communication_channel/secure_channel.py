# secure_channel.py
import socket
import ssl

class SecureChannel:
    def __init__(self, host: str, port: int, certfile: str, keyfile: str):
        self.host = host
        self.port = port
        self.certfile = certfile
        self.keyfile = keyfile

    def establish_connection(self) -> socket.socket:
        context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
        context.load_cert_chain(self.certfile, self.keyfile)
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        ssl_sock = context.wrap_socket(sock, server_hostname=self.host)
        ssl_sock.connect((self.host, self.port))
        return ssl_sock
