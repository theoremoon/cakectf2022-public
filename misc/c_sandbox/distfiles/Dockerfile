# docker build . -t c_sandbox
# docker run -it c_sandbox
FROM ubuntu:20.04

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -y update --fix-missing && apt-get -y upgrade
RUN apt-get -y install xinetd llvm-11 clang-11 python3
RUN groupadd -r ctf && useradd -r -g ctf ctf

WORKDIR /home/ctf
ADD libCSandbox.so .
ADD compiler.py .
RUN chmod 550 libCSandbox.so
RUN chmod 550 compiler.py

RUN chown -R root:ctf /home/ctf

CMD ["python3", "compiler.py"]
