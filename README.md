# Mina Mesh

[![checks](https://github.com/MinaFoundation/MinaMesh/actions/workflows/checks.yaml/badge.svg)](https://github.com/MinaFoundation/MinaMesh/actions/workflows/checks.yaml)

## Overview

Mina Mesh is an implementation of the
[Coinbase Mesh specification](https://docs.cdp.coinbase.com/mesh/docs/welcome) for the
[Mina blockchain](https://minaprotocol.com/).

## Building

To build the project:

```bash
cargo build
```

The binary will be available at:

```bash
target/debug/mina_mesh
```

## Running

Mina Mesh requires access to a PostgreSQL Archive database and a Mina GraphQL endpoint. By default,
the configuration points to the mainnet, making it easy to get started. You can override the
configuration by either passing arguments or setting environment variables via a `.env` file (an
example is provided as `.env.example`).

### Quick Start with Mainnet

1. **Set up the PostgreSQL Archive Database**

Use the predefined `just` commands to set up and start the PostgreSQL database:

```bash
just setup-archive-db
```

> Note: This process sets up the PostgreSQL docker using the latest mainnet archive database.

2. **Run the Mina Mesh Server**

To start the server with default settings (mainnet configuration):

```bash
target/debug/mina_mesh serve
```

The server will listen on `0.0.0.0:3000` by default.

### Playground Mode

You can enable a playground mode, which provides a simplified testing interface, by adding the
`--playground` flag:

```bash
cargo run -- serve --playground
```

When enabled, you can access the playground at the root URL (`/`).

### Configuration

Mina Mesh can be configured through command-line options or by using environment variables. For
convenience, you can use a `.env` file. To get started, copy the provided `.env.example`:

```bash
cp .env.example .env
```

Then modify the `.env` file to suit your environment. The available configurations include:

- **Mina GraphQL Endpoint**: `MINA_PROXY_URL` (default:
  `https://mainnet.minaprotocol.network/graphql`)
- **PostgreSQL Archive Database URL**: `MINA_ARCHIVE_DATABASE_URL` (default:
  `postgres://mina:whatever@localhost:5432/archive`)
- **Genesis Block Identifier**: `MINA_GENESIS_BLOCK_IDENTIFIER_HEIGHT`,
  `MINA_GENESIS_BLOCK_IDENTIFIER_STATE_HASH`

> You can also pass these options as arguments to `mina_mesh serve` to override the defaults.

## Testing

Running the tests requires having Archive database available [see:
[Quick Start with Mainnet](#quick-start-with-mainnet)]. Once the setup is complete you can run tests
using:

```bash
just test
```

### Managing PostgreSQL

- **Stop PostgreSQL**: To stop the PostgreSQL instance:

  ```bash
  just pg-down
  ```

- **Restart PostgreSQL**: To restart without reinitializing the database (useful if the database is
  already set up):

  ```bash
  just pg-up
  ```

> You only need to reinitialize the database if you want the latest data dump.
