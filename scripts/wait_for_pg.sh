#!/bin/bash

# This script is used to check if PostgreSQL is available
# Usage: ./scripts/pg_ready.sh <PG_HOST> <PG_PORT>
# Example: ./scripts/pg_ready.sh localhost 5432

# Parameters
PG_HOST="${1:-localhost}"
PG_PORT="${2:-5432}"

# Wait for PostgreSQL to become available
until pg_isready -h "$PG_HOST" -p "$PG_PORT"; do
  echo "Waiting for PostgreSQL to become available at ${PG_HOST}:${PG_PORT}..."
  sleep 1
done

echo "PostgreSQL is available at ${PG_HOST}:${PG_PORT}"