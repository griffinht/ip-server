# publish to crates.io
FROM rust

WORKDIR /usr/src

COPY ./ci/cargo/publish/publish.sh ./ci/cargo/publish/publish.sh
COPY README.md .
COPY Cargo.toml .
COPY src src

# make sure to pass in $CARGO_TOKEN as an argument
ENTRYPOINT ["./ci/cargo/publish/publish.sh"]