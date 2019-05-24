FROM ubuntu

RUN apt-get update
RUN apt-get install libssl1.0.0  openssl libssl-dev -y --fix-missing

ADD config.yml /bin
ADD ssl /bin/ssl
ADD server /bin/server
ADD static/med.txt /data/
ADD static/med1.txt /data/
ADD static/med2.txt /data/

WORKDIR /bin/

CMD server
