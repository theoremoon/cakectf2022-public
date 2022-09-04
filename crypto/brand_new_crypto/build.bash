#!/bin/bash
set -ex

export FLAG=CakeCTF{s0_anyway_tak3_car3_0f_0n3_byt3_p1aint3xt} 
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
