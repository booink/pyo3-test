# PyO3が吐き出すコードを眺めてみる

作業環境: rust:1.50-slim-buster

`cargo expand` するため nightly 版をインストールしています。

## 環境構築

```sh
docker-compose build
```

## 起動

```sh
docker-compose run --rm pyo3_test bash
```
