FROM rust:1.70.0-bullseye as Builder

RUN useradd --create-home --user-group ichiyo
USER ichiyo

WORKDIR /home/ichiyo/app
COPY --chown=ichiyo:ichiyo . .

RUN cargo build --release

FROM debian:bullseye-slim as Runner

COPY --from=Builder --chown=root:root /home/ichiyo/app/target/release/ichiyo_ai /usr/local/bin/ichiyo_ai

RUN useradd --create-home --user-group ichiyo
USER ichiyo
WORKDIR /home/ichiyo

LABEL org.opencontainers.image.source=https://github.com/approvers/ichiyoAI
ENTRYPOINT [ "ichiyo_ai" ]
