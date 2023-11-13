FROM quay.io/coredb/rust:1.71.0-slim-buster as builder

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && apt-get clean && \
    rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

WORKDIR /build

COPY . .

RUN cargo build --release

FROM quay.io/coredb/rust:1.71.0-slim-buster

# pgrep used for healthcheck
RUN apt-get update && apt-get install -y procps \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/webserver /usr/local/bin/webserver
