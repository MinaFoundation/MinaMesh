# Mina Mesh

Mina Mesh is an implementation of the
[Coinbase Mesh specification](https://docs.cdp.coinbase.com/mesh/docs/welcome) for the
[Mina blockchain](https://minaprotocol.com/).

> Note: Mina Mesh is WIP and should not yet be used in production.

## Installation

Ensure you have the Rust toolchain installed. If you do not, see
[installation instructions here](https://www.rust-lang.org/tools/install).

```sh
# Install the mina-mesh executable
cargo install mina-mesh

# Confirm installation successful
mina-mesh --help
```

## Fetch Genesis Block Identifier

Before running the server, we must first write genesis block identifier information to our `.env`
file.

```sh
mina-mesh fetch-genesis-block-identifier >> .env
```

> Note: this command utilizes a default GraphQL endpoint
> ("https://mainnet.minaprotocol.network/graphql"). You can override this default by specifying a
> `PROXY_URL` in your `.env` file.

## Instantiate the Server

Mina Mesh depends on a Postgres connection string for an archive database.

Ensure that you also specify an archive `DATABASE_URL` in your `.env`.

```sh
mina-mesh serve --playground
```

> Note: you may want to exclude the `--playground` flag in production. This will disable the
> playground, which renders when the server's root `/` route receives a GET request.

Alternatively, you can supply the archive database URL via the command line.

```sh
mina-mesh serve --playground --database-url postgres://mina:whatever@localhost:5432/archive
```

Then visit [`http://0.0.0.0:3000`](http://0.0.0.0:3000) for an interactive playground with which you
can explore and test endpoints.
