# publish to crates.io
FROM rust

WORKDIR /usr/src

COPY ./ci/cargo/publish.sh ./ci/cargo/publish.sh
COPY README.md .
COPY Cargo.toml .
COPY src src

ARG CARGO_TOKEN
RUN ./ci/cargo/publish.sh "$CARGO_TOKEN"

# install from crates.io
FROM rust as build

RUN cargo install ip-server

RUN ln -s /usr/local/cargo/bin/ip-server ip-server

# run installed binary
FROM gcr.io/distroless/cc

COPY --from=build /ip-server /usr/local/bin/ip-server

ENTRYPOINT ["ip-server"]