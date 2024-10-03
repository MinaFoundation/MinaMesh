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

## Environment

The server depends on several environment variables.

- `MINAMESH_PROXY_URL`: a Mina proxy (GraphQL) endpoint. The default is
  `https://mainnet.minaprotocol.network/graphql`.
- `MINAMESH_ARCHIVE_DATABASE_URL`: a connection string referencing a Mina archive database.
- `MINAMESH_GENESIS_BLOCK_IDENTIFIER_HEIGHT` and `MINAMESH_GENESIS_BLOCK_IDENTIFIER_STATE_HASH`: we
  can retrieve these using the `fetch-genesis-block-identifier` command.

  ```sh
  mina-mesh fetch-genesis-block-identifier >> .env
  ```

## Instantiate the Server

```sh
mina-mesh serve --playground
```

> Note: the presence of the `--playground` flag enables the serving of an OpenAPI playground in
> response to `GET /`. To disable this endpoint, omit the `--playground` flag.

Visit [`http://0.0.0.0:3000`](http://0.0.0.0:3000) for an interactive playground with which you can
explore and test endpoints.

## Code of Conduct

Everyone interacting in this repo is expected to follow the [code of conduct](CODE_OF_CONDUCT.md).

## Contributing

Contributions are welcome and appreciated! Check out the [contributing guide](CONTRIBUTING.md)
before you dive in.

## License

Mina Mesh is [Apache licensed](LICENSE).
