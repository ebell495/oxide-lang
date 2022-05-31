FROM rustlang/rust:nightly as builder
COPY . /oxide-lang
WORKDIR /oxide-lang
RUN cargo +nightly build

FROM debian:bullseye-slim
COPY --from=builder /oxide-lang/target/debug/oxide .