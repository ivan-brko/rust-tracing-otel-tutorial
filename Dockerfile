FROM rust:1.73-bookworm as builder

WORKDIR /usr/src/tracing_otel_tutorial
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /usr/src/tracing_otel_tutorial/target/release/tracing_otel_tutorial /app/tracing_otel_tutorial

CMD ["/app/tracing_otel_tutorial"]