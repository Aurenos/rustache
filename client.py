# Simple test client

import socket
import sys

msg = ""
if len(sys.argv) > 1:
    msg = " ".join(sys.argv[1:])

msg += "\n"

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("localhost", 8888))
sock.send(msg.encode())
response = sock.recv(1024)
sock.close()
print(response.decode())
