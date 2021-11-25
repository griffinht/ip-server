FROM rust as build

RUN cargo install cargo-deb

COPY src src
COPY Cargo.toml .
COPY README.md .
COPY LICENSE .

RUN ln -s "$(cargo deb)" /ip-server.deb

FROM debian

COPY --from=build /ip-server.deb .
RUN apt-get install ./ip-server.deb
RUN rm ./ip-server.deb

ENTRYPOINT ["ip-server"]