FROM rust as build

WORKDIR /usr/src/rust-ip

COPY . .
# cache dependencies
RUN cargo fetch
RUN cargo build

FROM debian

COPY --from=build /usr/src/rust-ip/target/debug/rust-ip /usr/local/bin/rust-ip

ENTRYPOINT rust-ip