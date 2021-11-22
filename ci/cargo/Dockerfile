FROM rust as build

WORKDIR /usr/src/ip-server

COPY Cargo.toml .
# cache build
# can't cache dependencies with vendor :(
COPY ./src src
RUN cargo build --release

# README.md needed for cargo publish
COPY README.md .

COPY ci/cargo/publish.sh .
ENTRYPOINT ["sh", "-c", "./publish.sh $CARGO_TOKEN"]