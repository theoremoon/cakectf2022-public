#!/bin/bash
set -ex

export FLAG=CakeCTF{oh_you_got_a_tepid_cake_sorry}
case $1 in
    clean)
        rm -rf ./distfiles
        ;;

    *)
        rm -rf ./distfiles
        mkdir -p ./distfiles
        python3 ./challenge/task.py > ./distfiles/output.txt
        cp ./challenge/task.py ./distfiles
        ;;
esac
