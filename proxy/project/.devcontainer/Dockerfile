FROM ubuntu:24.04

# 開発に必要なパッケージのインストール
RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config libssl-dev net-tools iputils-ping

# Rust インストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 作業ディレクトリ
WORKDIR /workspace
