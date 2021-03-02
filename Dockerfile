FROM rust:1.50

ENV CARGO_TERM_COLOR always
COPY / ./
RUN cargo install --path . --locked

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl
COPY --from=0 /usr/local/cargo/bin/telegram-bot-docker /usr/local/bin/telegram-bot-docker
CMD ["telegram-bot-docker"]
