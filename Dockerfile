FROM rust:bullseye

RUN cargo install cargo-compete \
    && rustup install 1.42.0 \
    && rustup default 1.42.0
