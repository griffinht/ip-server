FROM rust as build

WORKDIR /usr/src/ip-server

COPY Cargo.toml .
# cache build
# can't cache dependencies with vendor :(
COPY ./src src
RUN cargo build --release

COPY cargo/publish.sh .
COPY README.md .

CMD "./publish.sh"