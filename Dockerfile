ARG DEBIAN_CODENAME=bullseye

FROM debian:$DEBIAN_CODENAME AS builder

# Set environment variables
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install required dependencies and Rust in one step to minimize layers
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY .sqlx .sqlx
COPY sql sql
COPY src src
COPY static static
COPY build.rs Cargo.lock Cargo.toml ./

RUN cargo build --release

FROM debian:$DEBIAN_CODENAME-slim AS app

ARG MINA_BASE_TAG=3.0.3.1-cc59a03
ARG MINA_NETWORK=mainnet
ARG POSTGRES_VERSION=15
ARG DEBIAN_RELEASE_CHANNEL=stable

# Set environment variables
ENV MINA_NETWORK=$MINA_NETWORK
ENV MINA_BASE_TAG=$MINA_BASE_TAG
ENV DEBIAN_RELEASE_CHANNEL=$DEBIAN_RELEASE_CHANNEL
ENV POSTGRES_VERSION=$POSTGRES_VERSION

# Install dependencies and Mina daemon in one step
RUN apt-get update && apt-get install -y \
    curl \
    gnupg \
    sudo \
    postgresql-common \
    jq \
    && echo | /usr/share/postgresql-common/pgdg/apt.postgresql.org.sh \
    && apt-get install -y \
    postgresql-$POSTGRES_VERSION \
    && echo "deb [trusted=yes] http://packages.o1test.net $(grep VERSION_CODENAME /etc/os-release | cut -d= -f2) ${DEBIAN_RELEASE_CHANNEL}" | tee /etc/apt/sources.list.d/mina.list \
    && echo "Installing mina-${MINA_NETWORK}=${MINA_BASE_TAG}" \
    && apt-get update && \
    apt-get install --allow-downgrades -y mina-${MINA_NETWORK}=${MINA_BASE_TAG} mina-archive=${MINA_BASE_TAG} \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3087
COPY --from=builder /app/target/release/mina-mesh /usr/local/bin
COPY scripts /scripts
ENTRYPOINT ["/scripts/entrypoint.sh"]
