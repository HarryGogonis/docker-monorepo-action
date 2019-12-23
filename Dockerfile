FROM rust:1.40.0

ENV RUSTFLAGS "-C target-feature=-crt-static"
ENV GITHUB_WORKSPACE "/repo"

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["docker-monorepo-action"]
