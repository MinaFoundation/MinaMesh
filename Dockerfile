FROM rust

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY mesh_generated/ ./mesh_generated
COPY mina_mesh/ ./mina_mesh

RUN cargo build --release

ENTRYPOINT ["/app/target/release/mina-mesh", "serve"]
