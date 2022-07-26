# numeronym image

## 目的

- Dockerイメージを開発する経験を積む
- 開発用DockerコンテナからホストのDockerを使う方法を学ぶ

## やること

開発用Dockerコンテナ内でとあるRustコマンドを開発し、作成したコマンドを実行するDockerイメージ（の元になるDockerfile）を作成する。

作成するDockerイメージはセキュリティやサイズの観点から、最小限のベースイメージへstatic linkedなバイナリを配置する方針で行く。

ビルドする際に一々ホストへ移動するのが面倒なため、開発用Dockerコンテナ内でdocker buildできるようにする。今回はホストのDockerにイメージをビルドさせる方法をとる。所謂Docker outside of Docker(DooD)

## 作るコマンド

コマンドライン引数に指定した単語をヌメロニムにして出力する。

## Dockerイメージのビルドコマンド

プロジェクトルートで

```sh
docker build -t rusconn/numeronym:1.0.0 -f dockerfiles/prod/Dockerfile .
```

## 実行イメージ

```sh
$ docker container run --rm rusconn/numeronym:1.0.0 kubernetes
k8s
```
