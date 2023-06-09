FROM rust:latest 
RUN cargo new --bin projet
WORKDIR /projet
COPY . .
RUN cargo build --release 

FROM debian:stable-slim
COPY --from=0 /projet/target/release/api_codo_maton /
CMD ["./api_codo_maton"]

