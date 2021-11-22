# development Dockerfile
FROM rust as build

WORKDIR /usr/src/ip-server

COPY Cargo.toml .
# cache dependencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
# build
COPY ./src src
RUN cargo build --release
RUN ln -s $(cd ./target/release/; pwd)/ip-server /ip-server

# to use in other docker images
# COPY --from=ip-server /ip-server /usr/local/bin/ip-server

FROM gcr.io/distroless/cc

COPY --from=build /ip-server /usr/local/bin/ip-server

ENTRYPOINT ["ip-server"]