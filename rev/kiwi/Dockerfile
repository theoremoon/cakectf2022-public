FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -y update --fix-missing && apt-get -y upgrade
RUN apt-get -y install xinetd
RUN groupadd -r ctf && useradd -r -g ctf ctf

ADD challenge/ctf.xinetd /etc/xinetd.d/ctf
ADD challenge/init.sh    /etc/init.sh
ADD challenge/redir.sh   /home/ctf/.redir.sh
RUN chmod 550 /home/ctf/.redir.sh
RUN chmod 700 /etc/init.sh
RUN chmod 1733 /tmp /var/tmp /dev/shm

WORKDIR /home/ctf
ADD challenge/flag.txt flag.txt
ADD distfiles/chall    chall
RUN chmod 440 flag.txt
RUN chmod 550 chall

RUN chown -R root:ctf /home/ctf

RUN ls /home/ctf -lh

RUN service xinetd restart
