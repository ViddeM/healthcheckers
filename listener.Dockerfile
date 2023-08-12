FROM rust:1.70 as build

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y musl-tools 

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/healthcheck-listener

FROM scratch
WORKDIR /

ENV PORT=8080
EXPOSE 8080

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/healthcheck-listener ./healthcheck-listener

CMD ["./healthcheck-listener"]
