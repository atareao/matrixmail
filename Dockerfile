###############################################################################
## Builder
###############################################################################
FROM rust:alpine3.21 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN apk add --update --no-cache \
            autoconf \
            gcc \
            gdb \
            make \
            musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release && \
    cp /app/target/release/matrixmail /app/matrixmail

###############################################################################
## Final image
###############################################################################
FROM alpine:3.21

ENV USER=app
ENV UID=10001

RUN apk add --update --no-cache \
            ca-certificates \
            curl \
            openssl \
            tzdata~=2025 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy our build
COPY --from=builder /app/matrixmail /app/

# Set the work dir
WORKDIR /app
#USER app

CMD ["/app/matrixmail"]
