FROM ubuntu:20.04

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -y update --fix-missing && apt-get -y upgrade
RUN apt-get -y install xinetd llvm-11 clang-11 python3
RUN groupadd -r ctf && useradd -r -g ctf ctf

ADD challenge/ctf.xinetd /etc/xinetd.d/ctf
ADD challenge/init.sh    /etc/init.sh
ADD challenge/redir.sh   /home/ctf/.redir.sh
RUN chmod 550 /home/ctf/.redir.sh
RUN chmod 700 /etc/init.sh
RUN chmod 1733 /tmp /var/tmp /dev/shm

WORKDIR /home/ctf
ADD challenge/flag.txt flag.txt
ADD distfiles/libCSandbox.so .
ADD distfiles/compiler.py .
RUN chmod 440 flag.txt
RUN chmod 550 libCSandbox.so
RUN chmod 550 compiler.py
RUN mv flag.txt flag-$(md5sum flag.txt | awk '{print $1}').txt

RUN chown -R root:ctf /home/ctf

RUN ls /home/ctf -lh

RUN service xinetd restart
