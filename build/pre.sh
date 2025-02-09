#!/bin/bash

# Workaround for Rust 2021: https://github.com/ebbflow-io/cargo-deb-amd64-ubuntu/issues/1
rustup update stable
rustup default stable
rustup target add x86_64-unknown-linux-musl
