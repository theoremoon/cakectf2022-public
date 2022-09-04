from ptrlib import *
import os

HOST = os.getenv("HOST", "localhost")
PORT = int(os.getenv("PORT", "10011"))

sock = Socket(HOST, PORT)

sock.sendlineafter("Enter key: ", "01f389fbd70c0208000000000000000000")
l = sock.recvlineafter("Encrypted flag: ")
flag = ""
for i, c in enumerate(bytes.fromhex(l.decode())):
    flag += chr(c ^ i ^ 0xff)
print(flag)

sock.close()
