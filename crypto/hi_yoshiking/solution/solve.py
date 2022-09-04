from ptrlib import Socket, xor

sock = Socket("localhost", 10333)

sock.sendlineafter("> ", "1")
sock.sendlineafter("name: ", "yoshiking")
iv, token, tag = [bytes.fromhex(x) for x in sock.recvlineafter("token: ").strip().decode().split(":")]

key = xor(
    '{"username":"yoshiking","is_yoshiking":false}',
    '{"username":"yoshiking","is_yoshiking": true}',
)
token = xor(token, key)

known_tag = b""
for i in range(16):
    for tag in range(256):
        sock.sendlineafter("> ", "2")
        sock.sendlineafter("token: ", "{}:{}:{}".format(
            iv.hex(),
            token.hex(),
            (known_tag + bytes([tag])).hex(),
        ))

        line = sock.recvline().decode()
        if "CipherError" not in line:
            known_tag = known_tag + bytes([tag])
            if i == 15:
                print(line)
                quit()
            break


