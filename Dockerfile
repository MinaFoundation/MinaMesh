FROM rust

WORKDIR /app

COPY main.rs Cargo.toml Cargo.lock ./
COPY mesh_generated/ ./mesh_generated
COPY mina_mesh/ ./mina_mesh
COPY mina_mesh_server/ ./mina_mesh_server
COPY mina_mesh_graphql/ ./mina_mesh_graphql

RUN cargo build --release

ENTRYPOINT ["/app/target/release/mina-mesh", "serve"]
