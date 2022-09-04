#!/bin/sh
LENGTH=9
STRENGTH=26
challenge=`dd bs=32 count=1 if=/dev/urandom 2>/dev/null | base64 | tr +/ _. | cut -c -$LENGTH`
echo hashcash -mb$STRENGTH $challenge

echo "hashcash token: "
read token
if [ `expr "$token" : "^[a-zA-Z0-9\_\+\.\:\/]\{52\}$"` -eq 52 ]; then
    hashcash -cdb$STRENGTH -f /tmp/hashcash.sdb -r $challenge $token 2> /dev/null
    if [ $? -eq 0 ]; then
        echo "[+] Correct"
    else
        echo "[-] Wrong"
        exit
    fi
else
    echo "[-] Invalid token"
    exit
fi

cd /home/kosenctfx/welkerme/challenge/qemu
exec qemu-system-x86_64 \
     -m 64M \
     -nographic \
     -kernel bzImage \
     -append "console=ttyS0 loglevel=3 oops=panic panic=-1 nopti nokaslr" \
     -no-reboot \
     -cpu kvm64 \
     -monitor /dev/null \
     -initrd rootfs.cpio \
     -net nic,model=virtio \
     -net user
