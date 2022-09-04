#!/usr/bin/env python3
import os
import random

def tempname(extension='', length=16, directory='/tmp'):
    name = '{:x}'.format(random.randrange(0, 1<<(length*8)))
    return directory + '/' + name.zfill(length) + extension

def c_compile(code):
    c_path   = tempname(extension='.c')
    bc_path  = tempname(extension='.bc')
    ir_path  = tempname(extension='.ir')
    asm_path = tempname(extension='.asm')
    elf_path = tempname(extension='.bin')
    with open(c_path, 'w') as f:
        f.write(code)

    print("[+] Generating bitcode...")
    r = os.system('clang-11 -emit-llvm -c {} -o {} 2>/dev/null'
                  .format(c_path, bc_path))
    os.unlink(c_path)
    if r != 0: return

    print("[+] Instrumenting...")
    r = os.system('opt-11 -load ./libCSandbox.so -Sandbox < {} > {} 2>/dev/null'
                  .format(bc_path, ir_path))
    os.unlink(bc_path)
    if r != 0:
        os.unlink(ir_path)
        return

    print("[+] Translating to assembly...")
    r = os.system('llc-11 {} -o {} 2>/dev/null'
                  .format(ir_path, asm_path))
    os.unlink(ir_path)
    if r != 0: return

    print("[+] Compiling...")
    r = os.system('clang-11 {} -o {} 2>/dev/null'
                  .format(asm_path, elf_path))
    os.unlink(asm_path)
    if r != 0: return

    return elf_path

if __name__ == '__main__':
    print("Enter your C code (Type 'EOF' to quit input)")
    code = ''
    while True:
        line = input()
        if line == 'EOF': break
        code += line + '\n'
        if len(code) > 0x1000:
            print("[-] Too long")
            exit(1)

    elf_path = c_compile(code)
    if elf_path is None:
        print("[-] Compilation failed")
        exit(1)

    print("[+] Running...", flush=True)
    os.system("timeout -s KILL --foreground 60 {}".format(elf_path))
    os.unlink(elf_path)
