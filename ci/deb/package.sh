#!/bin/bash

cargo install cargo-deb
mv "$(cargo deb)" ./target/debian/ip-server.deb