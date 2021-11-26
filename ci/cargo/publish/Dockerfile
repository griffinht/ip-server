# publish to crates.io
FROM rust

WORKDIR /usr/src

COPY ./ci/cargo/publish/publish.sh ./ci/cargo/publish/publish.sh
COPY README.md .
COPY Cargo.toml .
COPY src src

ENTRYPOINT ["./ci/cargo/publish/publish.sh"]