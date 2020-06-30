FROM rust:1.44.1

WORKDIR /work
COPY / /work/
RUN cargo build --release
RUN cp -R /target/release /build
