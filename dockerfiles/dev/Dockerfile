FROM rust:1.62.1-slim-bullseye

# 開発コンテナ内でdockerコマンドを使うため、cli等をインストールしている
# https://docs.docker.com/engine/install/debian/#install-using-the-repository
RUN apt-get update \
  && apt-get install --no-install-recommends -y ca-certificates curl gnupg lsb-release \
  && mkdir -p /etc/apt/keyrings \
  && curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null \
  && apt-get update \
  && apt-get install -y docker-ce-cli docker-compose-plugin \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy
