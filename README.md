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

## Testing

### Setup PostgreSQL with Latest Mainnet Archive DB

To set up the testing environment with a working PostgreSQL database, use the predefined `just`
steps:

```bash
just get-mainnet-archive-db
just pg
just wait-for-pg
```

> Note: This process sets up the environment using the latest mainnet archive database.

### Run Tests

Once the setup is complete, run the tests with:

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
