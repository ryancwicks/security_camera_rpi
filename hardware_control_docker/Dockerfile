FROM rust:1.58 as builder

RUN rustup target add armv7-unknown-linux-musleabihf
RUN apt-get update && apt-get -y install binutils-arm-linux-gnueabihf libssl1.1 libssl-dev gcc-10-arm-linux-gnueabihf musl-tools

WORKDIR /app
COPY .cargo ./.cargo
COPY Cargo.toml ./
RUN mkdir src
RUN echo //null > src/lib.rs
RUN CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld REALGCC=arm-linux-gnueabihf-gcc-10 TARGET_CC=musl-gcc cargo build --release --target armv7-unknown-linux-musleabihf
RUN cargo build --release --target armv7-unknown-linux-musleabihf
RUN rm src -rf

COPY src ./src
RUN CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld REALGCC=arm-linux-gnueabihf-gcc-10 TARGET_CC=musl-gcc cargo build --release --target armv7-unknown-linux-musleabihf
RUN cargo build --release --target armv7-unknown-linux-musleabihf

#This creates a new image, but for now we're just copying out the build file.
#FROM --platform=linux/arm alpine:3.15
#WORKDIR /app
#COPY --from=builder /app/target/armv7-unknown-linux-musleabihf/release/hardware_control ./
#COPY --from=builder /app/target/armv7-unknown-linux-musleabihf/release/hardware_test ./

#CMD ["hardware_test"]

#This copies the built file out of the container
FROM scratch AS export-stage
COPY --from=builder /app/target/armv7-unknown-linux-musleabihf/release/hardware_control ./
#COPY --from=builder /app/target/armv7-unknown-linux-musleabihf/release/hardware_test ./