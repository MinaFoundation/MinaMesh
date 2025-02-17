# syntax = docker/dockerfile:1.4

ARG MINA_ROSETTA_IMAGE

FROM ubuntu:20.04 AS builder

RUN set -eux; \
		apt update; \
		apt install -y --no-install-recommends curl ca-certificates gcc pkg-config libssl-dev libc6-dev;

# Install rustup
RUN --mount=type=cache,target=/root/.rustup \
    set -eux; \
		curl --location --fail \
			"https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
			--output rustup-init; \
		chmod +x rustup-init; \
		./rustup-init -y --no-modify-path --default-toolchain stable; \
		rm rustup-init;

# Add rustup to path, check that it works
ENV PATH=${PATH}:/root/.cargo/bin
RUN set -eux; \
		rustup --version;

WORKDIR /app
RUN mkdir -p ./sql ./src ./static ./.sqlx
COPY .sqlx .sqlx
COPY sql sql
COPY src src
COPY static static
COPY build.rs Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
		--mount=type=cache,target=/app/target \
		set -eux; \
		cargo build --release; \
		cp /app/target/release/mina-mesh /app

FROM ${MINA_ROSETTA_IMAGE} as app

SHELL ["/bin/bash", "-c"]

RUN set -eux; \
		apt update; \
		apt install -y --no-install-recommends \
			ca-certificates \
			; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,cache,log}/

COPY --from=builder /app/mina-mesh /usr/local/bin

WORKDIR /etc/mina/rosetta/scripts

COPY scripts/mina-mesh-standalone.sh .

ENTRYPOINT ["bash", "./mina-mesh-standalone.sh"]
