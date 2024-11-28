FROM rust:alpine AS builder 
WORKDIR /home 
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN apk add musl-dev
RUN cargo build --release
RUN ["sh"]

FROM alpine:3.20 AS runtime 
WORKDIR /home 
COPY --from=builder /home/target/release/purplerag_db_api /home
COPY .env .
RUN source .env
EXPOSE $SERVER_PORT
ENTRYPOINT [ "/home/purplerag_db_api" ]
