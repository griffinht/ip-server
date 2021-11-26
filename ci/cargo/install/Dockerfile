# install from crates.io
FROM rust as build

ARG VERSION
RUN cargo install ip-server --version "$VERSION"

RUN ln -s /usr/local/cargo/bin/ip-server ip-server

# run installed binary
FROM gcr.io/distroless/cc

COPY --from=build /ip-server /usr/local/bin/ip-server

ENTRYPOINT ["ip-server"]