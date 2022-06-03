
FROM --platform=linux/arm/v7 rust:1.59.0 as builder

RUN mkdir /usr/src/auth-service
WORKDIR /usr/src/auth-service
COPY . .

RUN apt-get update -yq 
RUN apt install openssl libssl-dev clang llvm-dev libclang-dev -yq

RUN rustup default 1.59.0
RUN cargo build --release

EXPOSE 3000



FROM --platform=linux/arm/v7 rust:1.59.0

RUN cargo install --git https://github.com/launchbadge/sqlx sqlx-cli

COPY --from=builder /usr/src/auth-service/target/release/auth-service /usr/src/auth-service/
COPY --from=builder /usr/src/auth-service/.env /usr/src/.env
COPY --from=builder /usr/src/auth-service/migrations /usr/src/migrations

WORKDIR /usr/src/auth-service

EXPOSE 3000

CMD ["./auth-service"]
