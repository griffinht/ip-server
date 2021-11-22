FROM rust as build

WORKDIR /usr/src/rust-ip

COPY Cargo.toml .
# cache dependencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
# build
COPY ./src src
RUN cargo build --release
RUN ln -s $(cd ./target/release/; pwd)/rust-ip /rust-ip

# to use in other docker images
# COPY --from=rust-ip /rust-ip /usr/local/bin/rust-ip

FROM gcr.io/distroless/cc

COPY --from=build /rust-ip /usr/local/bin/rust-ip

ENTRYPOINT ["rust-ip"]