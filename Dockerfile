FROM rust:1.46-alpine as build

RUN apk add --no-cache g++ make postgresql-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /var/usr/app

RUN USER=root cargo init .

COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release -j $(nproc) --target x86_64-unknown-linux-musl

COPY ./src src
RUN touch ./src/main.rs
RUN cargo build --release -j $(nproc) --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /var/usr/app

COPY ./migrations ./migrations

COPY --from=build /var/usr/app/target/x86_64-unknown-linux-musl/release/discord-logger .

ENTRYPOINT [ "./discord-logger" ]