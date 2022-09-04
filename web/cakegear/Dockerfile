FROM php:8-apache

ADD challenge/flag.txt /
RUN chmod 444 /flag.txt

RUN apt update

WORKDIR /var/www/html

ADD distfiles/index.php ./
ADD distfiles/admin.php ./
RUN chown -R root:www-data ./
RUN chmod -R 440           ./
RUN chmod    550           ./
