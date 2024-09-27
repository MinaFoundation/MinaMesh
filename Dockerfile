FROM rust AS build
WORKDIR /app

COPY rustfmt.toml .
COPY rust-toolchain.toml .
RUN cargo --help

COPY Cargo.toml Cargo.lock ./
COPY mesh_generated/ mesh_generated
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY build.rs .
COPY sql_fmt.json .
COPY sql sql
COPY .sqlx .sqlx
COPY static static
COPY src src
RUN cargo build --release

# Todo: fix the SQL connection string at build
# Todo: remove the need of .env

RUN touch .env

FROM rust
COPY --from=build /app/target/release/mina_mesh /usr/local/bin/mina-mesh
COPY --from=build /app/.env /.env
ENTRYPOINT ["mina-mesh"]
CMD ["serve"]
