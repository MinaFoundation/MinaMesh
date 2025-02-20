#!/usr/bin/env bash

set -e
set -o pipefail

# Setting default values
export MINA_NETWORK="${MINA_NETWORK:=mainnet}"
export MINA_GENESIS_LEDGER_URL="${MINA_GENESIS_LEDGER_URL:-}"
export MINA_CONFIG_FILE="${MINA_CONFIG_FILE:-/var/lib/coda/${MINA_NETWORK}.json}"
export POSTGRES_HOST="${POSTGRES_HOST:-127.0.0.1}"
export POSTGRES_PORT="${POSTGRES_PORT:-5432}"
export POSTGRES_USERNAME="${POSTGRES_USERNAME:-pguser}"
export POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-${POSTGRES_USERNAME}}"
export POSTGRES_DBNAME="${POSTGRES_DBNAME:-archive}"
export POSTGRES_CONNECTION_STRING="postgres://${POSTGRES_USERNAME}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DBNAME}"
export POSTGRES_DATA_DIR="${POSTGRES_DATA_DIR:-/data/postgresql}"
export MINA_ARCHIVE_DUMP_TIME="${MINA_ARCHIVE_DUMP_TIME:-0000}"
export MINA_ARCHIVE_DUMP_URL="${MINA_ARCHIVE_DUMP_URL:-https://673156464838-mina-archive-node-backups.s3.us-west-2.amazonaws.com}"
export PRECOMPUTED_BLOCKS_URL="${PRECOMPUTED_BLOCKS_URL:-https://673156464838-mina-precomputed-blocks.s3.us-west-2.amazonaws.com}"
export MINA_ARCHIVE_PORT="${MINA_ARCHIVE_PORT:-3086}"
export MINA_ARCHIVE_LOG_LEVEL="${MINA_ARCHIVE_LOG_LEVEL:-Error}"
export PEER_LIST_URL="${PEER_LIST_URL:-https://bootnodes.minaprotocol.com/networks/${MINA_NETWORK}.txt}"
export MINA_GRAPHQL_PORT="${MINA_GRAPHQL_PORT:=3085}"
export MINA_GRAPHQL_URL=http://127.0.0.1:$MINA_GRAPHQL_PORT/graphql
export MINA_DAEMON_LOG_LEVEL="${MINA_DAEMON_LOG_LEVEL:-Error}"
export MINA_EXTRA_FLAGS="${MINA_EXTRA_FLAGS:-}"
export MINA_DAEMON_PID=""
export MINA_MESH_HOST="${MINA_MESH_HOST:-0.0.0.0}"
export MINA_MESH_ONLINE_PORT="${MINA_MESH_ONLINE_PORT:-3087}"
export MINA_MESH_OFFLINE_MODE="${MINA_MESH_OFFLINE_MODE:-enabled}"
export MINA_MESH_OFFLINE_PORT="${MINA_MESH_OFFLINE_PORT:-3088}"
export MINA_MESH_LOG_LEVEL="${MINA_MESH_LOG_LEVEL:-Info}"

# Fetch Genesis Ledger
/scripts/get-genesis-ledger.sh

# Initialize Database
/scripts/init-db.sh

# Start Mina Archive
/scripts/start-mina-archive.sh

# Start Mina Daemon
/scripts/start-mina-daemon.sh

# Start Mina Mesh (Online)
SEARCH_TX_OPTIMIZATIONS=enabled MINA_MESH_PORT=$MINA_MESH_ONLINE_PORT /scripts/start-mina-mesh.sh

# Start Mina Mesh (Offline) if enabled
[ "$MINA_MESH_OFFLINE_MODE" = "enabled" ] && \
SEARCH_TX_OPTIMIZATIONS=enabled MINA_MESH_PORT=$MINA_MESH_OFFLINE_PORT /scripts/start-mina-mesh.sh

echo "$MINA_DAEMON_PID"

sleep infinity
