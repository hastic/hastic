FROM rust:1.57.0-bullseye as builder

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
    nodejs \
    gcc \
    g++ \
    make \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN npm install --global yarn

RUN rustup target add x86_64-unknown-linux-musl

ADD . ./

RUN make

FROM debian:bullseye-slim

COPY --from=builder /release/hastic /hastic
COPY --from=builder /release/config.toml /config.toml
COPY --from=builder /release/public /public

CMD ["./hastic"]
