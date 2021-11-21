FROM rust as build

WORKDIR /usr/src/rust-ip

COPY Cargo.toml .
# cache dependencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
# build
COPY ./src src
RUN cargo build --release
COPY target/release/rust-ip /rust-ip

FROM debian

COPY --from=build /rust-ip /usr/local/bin/rust-ip

ENTRYPOINT rust-ip

# to use in other docker files
# COPY --from=rust-ip /rust-ip rust-ip