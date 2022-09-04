#!/bin/bash
TIMEOUT=600
HOST="web2.2022.cakectf.com"
DOCKER_IMAGE="panda"

cat <<EOF
     .::.::.::.
    ::^^:^^:^^::  >> Cake Chef <<
    ':'':'':'':'  
    _i__i__i__i_     CTF
   (____________)    Instance
   |#o##o##o##o#|    Generator
   (____________)
We will create a new server for you.
 Please test your exploit locally.

EOF

LENGTH=10
STRENGTH=26
challenge=`dd bs=32 count=1 if=/dev/urandom 2>/dev/null | base64 | tr +/ _. | cut -c -$LENGTH`
echo hashcash -mb$STRENGTH $challenge

# Challenge
echo "hashcash token: "
read token
if [ `expr "$token" : "^[a-zA-Z0-9\+\_\.\:\/]\{52\}$"` -eq 52 ]; then
    hashcash -cdb$STRENGTH -f /tmp/hashcash.sdb -r $challenge $token 2> /dev/null
    if [ $? -eq 0 ]; then
        echo "[+] Correct"
    else
        echo "[-] Wrong"
        exit
    fi
else
    echo "[-] Wrong"
    exit
fi

# Get random port
BASIC_USERNAME="guest"
BASIC_PASSWORD=`dd bs=32 count=1 if=/dev/urandom 2>/dev/null | base64 | tr +/ _. | cut -c -16`
read LOWERPORT UPPERPORT < /proc/sys/net/ipv4/ip_local_port_range
while :
do
    PORT="`shuf -i $LOWERPORT-$UPPERPORT -n 1`"
    ss -lpn | grep -q ":$PORT " || break
done

echo "Server: http://${HOST}:${PORT}/"
echo "Username of Basic Auth: ${BASIC_USERNAME}"
echo "Password of Basic Auth: ${BASIC_PASSWORD}"
echo "Timeout: ${TIMEOUT}sec"
echo "It may take less than a minute to start a new instance."
echo "Please be patient. You can close this connection now."

cd /home/kosenctfx/panda_memo
timeout --foreground -s9 "${TIMEOUT}" \
        docker run \
        --env-file=./secret.txt \
        --env-file=./flag.txt \
        -e BASIC_USERNAME="${BASIC_USERNAME}" \
        -e BASIC_PASSWORD="${BASIC_PASSWORD}" \
        -p "${PORT}:3000" \
        --rm --name "${challenge}" "${DOCKER_IMAGE}" 2>/dev/null 1>/dev/null
echo "Timeout!"
docker kill "${challenge}" 2>&1 1>/dev/null
