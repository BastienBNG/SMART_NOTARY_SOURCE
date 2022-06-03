FROM --platform=linux/arm/v7 rust:1.59.0 as builder

RUN mkdir /usr/src/rustwebservice
WORKDIR /usr/src/rustwebservice
COPY . .

RUN rustup default nightly-2022-04-25
RUN cargo build --release

EXPOSE 8000

FROM --platform=linux/arm/v7 gcr.io/distroless/cc-debian11


COPY --from=builder /usr/src/rustwebservice/target/release/rustwebservice /usr/src/rustwebservice/
COPY --from=builder /usr/src/rustwebservice/templates /usr/src/rustwebservice/templates/
COPY --from=builder /usr/src/rustwebservice/static /usr/src/rustwebservice/static/
COPY --from=builder /usr/src/rustwebservice/pkg /usr/src/rustwebservice/pkg/


WORKDIR /usr/src/rustwebservice

EXPOSE 8000

CMD ["./rustwebservice"]
