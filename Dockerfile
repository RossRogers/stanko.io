FROM debian:jessie

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
      wget \
      curl \
      gdb \
      g++-multilib \
      lib32stdc++6 \
      libssl-dev \
      libncurses5-dev \
      make \
      ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENV SSL_CERT_FILE /etc/ssl/certs/ca-certificates.crt
RUN mkdir -p /usr/local/share/ca-certificates/cacert.org \
    && wget -P /usr/local/share/ca-certificates/cacert.org \
      http://www.cacert.org/certs/root.crt \
      http://www.cacert.org/certs/class3.crt \
    && update-ca-certificates

RUN echo "deb http://apt.postgresql.org/pub/repos/apt/ jessie-pgdg main" \
    > /etc/apt/sources.list.d/pgdg.list
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc \
    | apt-key add -
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
      postgresql-9.6 \
      postgresql-contrib-9.6 \
      libpq-dev \
    && rm -rf /var/lib/apt/lists/*

ENV USER root
ENV TRIPLET x86_64-apple-darwin
ENV RUST_VERSION_DATE 2017-04-28
ENV RUST_DOWNLOAD_LINK https://static.rust-lang.org/dist/$RUST_VERSION_DATE/rust-nightly-$TRIPLET.tar.gz
ENV CARGO_DOWNLOAD_LINK https://static.rust-lang.org/cargo-dist/cargo-nightly-$TRIPLET.tar.gz

RUN curl -sL $RUST_DOWNLOAD_LINK \
    | tar xvz -C /tmp \
    && /tmp/rust-nightly-$TRIPLET/install.sh
RUN curl -sL $CARGO_DOWNLOAD_LINK \
    | tar xvz -C /tmp \
    && /tmp/cargo-nightly-$TRIPLET/install.sh

WORKDIR /app

COPY src src
COPY .env.test .env
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY Rocket.toml Rocket.toml
COPY start.sh start.sh

RUN cargo build --release
RUN rm -rf src/

EXPOSE 80

CMD ["./start.sh"]
