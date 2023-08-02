FROM rust:1.71.0-bullseye as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin ichiyo_ai

FROM debian:bullseye-slim as Runner

RUN useradd --create-home --user-group ichiyo

COPY --from=Builder --chown=root:root /root/app/target/release/ichiyo_ai /usr/local/bin/ichiyo_ai
COPY --from=Builder --chown=root:root /root/app/resource /home/ichiyo/resource

USER ichiyo
WORKDIR /home/ichiyo

LABEL org.opencontainers.image.source=https://github.com/approvers/ichiyoAI
ENTRYPOINT [ "ichiyo_ai" ]
