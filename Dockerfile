FROM rust:1.38-stretch as builder

ADD . ./

RUN apt update && \
    cargo build --verbose --release && \
    cargo install
