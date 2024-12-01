FROM rust:1.81.0

# ビルドに必要なパッケージをインストール
RUN apt-get update && \
    apt-get install -y curl libssl-dev pkg-config build-essential

WORKDIR /app

COPY . .

RUN cargo build
RUN cargo install sqlx-cli

EXPOSE 3478

CMD ["cargo", "run"]