FROM rust:1.56.0-alpine as builder

RUN mkdir /build
ADD Cargo.toml Cargo.lock build.rs /build/
ADD src /build/src/
WORKDIR /build
RUN apk add --no-cache musl-dev cmake make protoc
RUN cargo build --release

FROM scratch
COPY --from=builder /build/target/release/sparkplug-simulator /
CMD ["/sparkplug-simulator"]
