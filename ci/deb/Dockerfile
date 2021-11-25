FROM debian

COPY target/debian/ip-server.deb .

RUN apt-get install ./ip-server.deb
RUN rm ./ip-server.deb

ENTRYPOINT ["ip-server"]