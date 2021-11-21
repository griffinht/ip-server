FROM rust as build

WORKDIR /usr/src/rust-ip

COPY Cargo.toml .
# cache dependencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
# build
COPY ./src src
RUN cargo build --release

FROM debian

COPY --from=build /usr/src/rust-ip/target/release/rust-ip /usr/local/bin/rust-ip

ENTRYPOINT rust-ip