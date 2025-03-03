#!/usr/bin/env bash

set -euxo pipefail

# Function to log environment variables
log_environment() {
    echo "Starting Mina Mesh with the following configurations:"
    echo "  PostgreSQL host: $POSTGRES_HOST"
    echo "  Mina Mesh host: $MINA_MESH_HOST"
    echo "  Mina Mesh Port: $MINA_MESH_PORT"
    echo "  Mina GraphQL Endpoint: $MINA_GRAPHQL_URL"
}

# Function to start Mina Mesh server
start_mina_mesh() {
    /usr/local/bin/mina-mesh serve \
      --archive-database-url "$POSTGRES_CONNECTION_STRING" \
      --proxy-url "$MINA_GRAPHQL_URL" \
      "$MINA_MESH_HOST" \
      "$MINA_MESH_PORT" &
    sleep 10
}

apply_search_tx_optimizations() {
    if /usr/local/bin/mina-mesh search-tx-optimizations \
        --archive-database-url "$POSTGRES_CONNECTION_STRING" \
        --check; then
        echo "Applying transaction optimizations..."
        /usr/local/bin/mina-mesh search-tx-optimizations \
          --archive-database-url "$POSTGRES_CONNECTION_STRING" \
          --apply
    else
        echo "No optimizations needed."
    fi
}

# Main execution
log_environment

# Apply search transaction optimizations if enabled
[ "$SEARCH_TX_OPTIMIZATIONS" = "enabled" ] && apply_search_tx_optimizations

start_mina_mesh
