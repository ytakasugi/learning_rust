FROM rust:1.60-buster

RUN apt-get update && \
    apt-get -y install git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch && \
    cargo install diesel_cli --no-default-features --features postgres

WORKDIR /workspace

COPY ./app ./workspace/app
