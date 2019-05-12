FROM ubuntu

ADD server /bin/server
ADD static/med.txt /data/
ADD cert.pem /etc/cert.pem
ADD key.pem /etc/key.pem

RUN apt-get update
RUN apt-get install libssl1.0.0 libssl-dev -y

CMD /bin/server
