FROM rust:1.81.0

# ビルドに必要なパッケージをインストール
RUN apt-get update && \
    apt-get install -y curl libssl-dev pkg-config build-essential

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY .env .env

RUN cargo build

EXPOSE 3478

CMD ["cargo", "run"]