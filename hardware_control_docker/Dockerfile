FROM rust:1.56 as builder

RUN USER=root cargo new --bin hardware_control
WORKDIR ./hardware_control
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hardware_control /usr/local/bin/hardware_control
CMD ["hardware_control"]
