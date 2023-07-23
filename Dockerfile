FROM rust:latest as build

RUN USER=root cargo new --bin umfrager
WORKDIR umfrager/

COPY ./Cargo.toml ./Cargo.toml

# chaches dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src/ ./src/

RUN rm ./target/release/deps/umfrager*
RUN cargo build --release

FROM rust:slim-buster

RUN apt-get update
RUN apt-get install -y sqlite3

COPY --from=build /umfrager/target/release/umfrager .

COPY ./diesel.toml ./diesel.toml
COPY ./.env ./.env
COPY ./survey.db ./survey.db
COPY ./migrations/ ./migrations/
COPY ./static/ ./static/

CMD ["./umfrager"]
