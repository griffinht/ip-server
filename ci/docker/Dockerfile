# build from source
FROM rust as build

COPY src src
COPY Cargo.toml .
RUN cargo install --path .
RUN ln -s /usr/local/cargo/bin/ip-server ip-server
# runtime
FROM gcr.io/distroless/cc

COPY --from=build /ip-server /usr/local/bin/ip-server

ENTRYPOINT ["ip-server"]