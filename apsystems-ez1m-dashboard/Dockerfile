FROM rust:1.76.0-alpine3.19 as build

RUN USER=root cargo new --bin apsystems-ez1m-dashboard
WORKDIR /apsystems-ez1m-dashboard

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build the application
RUN rm ./target/release/deps/apsystems_ez1m_dashboard*
RUN cargo build --release

# Copy the binary to a new container
FROM alpine:3.19.1

RUN addgroup -S apsystems-ez1m-dashboard && adduser -S apsystems-ez1m-dashboard -G apsystems-ez1m-dashboard

COPY --from=build /apsystems-ez1m-dashboard/target/release/apsystems-ez1m-dashboard /usr/local/bin/apsystems-ez1m-dashboard

USER apsystems-ez1m-dashboard

CMD ["/usr/local/bin/apsystems-ez1m-dashboard"] 
