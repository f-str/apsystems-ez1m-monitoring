FROM rust:1.76.0-alpine3.19 as build

RUN USER=root cargo new --bin weather-logger 
WORKDIR /weather-logger

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build the application
RUN rm ./target/release/deps/weather_logger*
RUN cargo build --release

# Copy the binary to a new container
FROM alpine:3.19.1

RUN addgroup -S weather-logger && adduser -S weather-logger -G weather-logger

COPY --from=build /weather-logger/target/release/weather-logger /usr/local/bin/weather-logger

USER weather-logger

CMD ["/usr/local/bin/weather-logger"] 
