# production dockerfile
FROM rust as build

ARG CARGO_VERSION

RUN cargo install ip-server --version "${CARGO_VERSION}"

RUN ln -s /usr/local/cargo/bin/ip-server ip-server

FROM gcr.io/distroless/cc

COPY --from=build /ip-server /usr/local/bin/ip-server

ENTRYPOINT ["ip-server"]