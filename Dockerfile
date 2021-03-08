FROM rust:1.50-slim-buster

RUN apt-get update -y \
    && apt-get install -y libssl-dev pkg-config locales \
    && locale-gen ja_JP.UTF-8

RUN localedef -f UTF-8 -i ja_JP ja_JP.UTF-8
ENV LANG="ja_JP.UTF-8" \
    LANGUAGE="ja_JP:ja" \
    LC_ALL="ja_JP.UTF-8"

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     git \
     wget \
     default-libmysqlclient-dev \
     curl \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     python3 \
     python3-pip \
     ruby \
     ruby-dev \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN rustup component add clippy
RUN cargo install cargo-watch cargo-make cargo-expand cargo-clippy

WORKDIR /app
COPY Cargo.toml Cargo.toml
ADD src src
