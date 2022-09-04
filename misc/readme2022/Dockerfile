FROM python:3.10-slim-buster

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get install -yqq socat
RUN groupadd -r ctf && useradd -r -g ctf ctf

RUN echo "CakeCTF{~USER_r3f3rs_2_h0m3_d1r3ct0ry_0f_USER}" > /flag.txt
RUN chmod 444 /flag.txt

USER ctf
WORKDIR /app
ADD distfiles/server.py .

CMD socat TCP-L:9999,fork,reuseaddr EXEC:"python server.py",pty,ctty,stderr,echo=0
