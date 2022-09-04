FROM ruby:3-slim-bullseye

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get install -yqq socat
ADD challenge/server.rb server.rb

ENV flag "CakeCTF{hi_yoshiking_lets_go_sushi}"
CMD socat TCP-L:9999,fork,reuseaddr EXEC:"ruby server.rb"



