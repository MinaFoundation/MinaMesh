# Contributing

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
