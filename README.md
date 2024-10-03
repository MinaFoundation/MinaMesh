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

# Confirm the installation was successful
mina-mesh --help
```

## Fetch Genesis Block Identifier

The server depends on two environment variables (the genesis block identifier hash and index). We
can retrieve these using the `fetch-genesis-block-identifier` command.

```sh
mina-mesh fetch-genesis-block-identifier >> .env
```

> Note: all commands utilize a default GraphQL endpoint
> ("https://mainnet.minaprotocol.network/graphql"). You can override this default by specifying the
> `PROXY_URL`.

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

## Code of Conduct

Everyone interacting in this repo is expected to follow the [code of conduct](CODE_OF_CONDUCT.md).

## Contributing

Contributions are welcome and appreciated! Check out the [contributing guide](CONTRIBUTING.md)
before you dive in.

## License

Mina Mesh is [Apache licensed](LICENSE).
