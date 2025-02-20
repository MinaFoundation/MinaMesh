#!/usr/bin/env bash

set -e
set -o pipefail

# Function to log the environment variables
log_environment() {
    echo "Starting Mina Archive with the following configurations:"
    echo "  PostgreSQL host: $POSTGRES_HOST"
    echo "  Log level: $MINA_ARCHIVE_LOG_LEVEL"
    echo "  Archive port: $MINA_ARCHIVE_PORT"
}

# Function to define and start Mina Archive with the necessary flags
start_mina_archive() {
    mina-archive run \
        --postgres-uri "$POSTGRES_CONNECTION_STRING" \
        --log-level "$MINA_ARCHIVE_LOG_LEVEL" \
        --log-json \
        --server-port "$MINA_ARCHIVE_PORT" &
    sleep 10
}

# Main execution
log_environment
start_mina_archive
