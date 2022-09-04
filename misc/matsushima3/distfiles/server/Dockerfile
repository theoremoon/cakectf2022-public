FROM tiangolo/uwsgi-nginx-flask:python3.8-alpine

RUN apk update
RUN adduser -D ctf
RUN pip install Flask Flask-Session redis

WORKDIR /home/ctf
ADD . .
RUN chown -R root:ctf .
