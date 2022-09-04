#!/bin/bash

#!/bin/bash
set -ex

case $1 in
    clean)
        rm -rf ./distfiles
        ;;

    *)
        rm -rf ./distfiles
        mkdir -p ./distfiles
        cp ./challenge/server.py ./distfiles/server.py
        ;;
esac
