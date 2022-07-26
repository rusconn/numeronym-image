FROM ekidd/rust-musl-builder:stable as builder
WORKDIR /app
# キャッシュのために依存ライブラリだけをビルドする
COPY Cargo.toml Cargo.lock ./
# プログラムが無い場合のビルドエラーを回避するワークアラウンド
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
# 本物のビルドを行う
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /numeronym
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/numeronym . 
ENTRYPOINT [ "./numeronym" ]
