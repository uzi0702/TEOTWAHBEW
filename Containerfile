# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.96.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# ------------------------------
# Stage 2. Build for runtime
# ------------------------------
FROM dhi.io/debian-base:trixie

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION
LABEL org.opencontainers.image.title="teot" \
      org.opencontainers.image.description="TEOTWAHBEW is a CLI command like 'ls'" \
      org.opencontainers.image.url="https://uzi0702.github.io/TEOTWAHBEW" \
      org.opencontainers.image.source="https://github.com/uzi0702/TEOTWAHBEW" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.licenses="CC0-1.0"

COPY --from=builder /app/target/release/teot /app/teot
WORKDIR /opt
ENTRYPOINT [ "/app/teot" ]
