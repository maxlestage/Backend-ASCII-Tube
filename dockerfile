FROM rust:latest 
RUN cargo new --bin projet
WORKDIR /projet
COPY ./api/. /projet/api
COPY ./auth/. /projet/auth
COPY ./db/. /projet/db
COPY ./entities/. /projet/entities
COPY ./migration/. /projet/migration
COPY ./queries/. /projet/queries
COPY ./src/. /projet/src
COPY ./Cargo.toml /projet/
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release 

FROM debian:stable-slim
COPY --from=0 /projet/target/release/api_codo_maton /
CMD ["./api_codo_maton"]
