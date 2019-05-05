FROM ubuntu

RUN apt-get update
RUN apt-get install libssl1.0.0 libssl-dev -y

ADD server /bin/server
ADD static/med.txt /data/

CMD /bin/server
