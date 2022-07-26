FROM numeronym-dev:1.0.0

RUN apt-get update \
  && apt-get install --no-install-recommends -y git fish \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
