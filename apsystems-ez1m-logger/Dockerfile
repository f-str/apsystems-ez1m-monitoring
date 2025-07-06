FROM rust:1.88.0-alpine3.20 AS build

RUN apk add musl-dev openssl-dev perl make

RUN USER=root cargo new --bin apsystems-ez1m-logger
WORKDIR /apsystems-ez1m-logger

COPY ./apsystems-ez1m-logger/Cargo.toml ./Cargo.toml
COPY ./apsystems-ez1m-logger/Cargo.lock ./Cargo.lock

# Copy the utils package
COPY ./apsystems-ez1m-util ../apsystems-ez1m-util

# Copy the migration
COPY ./migrations ../migrations

RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./apsystems-ez1m-logger/src ./src

# Build the application
RUN rm ./target/release/deps/apsystems_ez1m_logger*
RUN cargo build --release

# Copy the binary to a new container
FROM alpine:3.20

RUN addgroup -S apsystems-ez1m-logger && adduser -S apsystems-ez1m-logger -G apsystems-ez1m-logger

COPY --from=build /apsystems-ez1m-logger/target/release/apsystems-ez1m-logger /usr/local/bin/apsystems-ez1m-logger

USER apsystems-ez1m-logger

CMD ["/usr/local/bin/apsystems-ez1m-logger"] 
