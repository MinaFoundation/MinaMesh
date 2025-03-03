#!/usr/bin/env bash

set -euxo pipefail

# Fetch Genesis Ledger
/scripts/get-genesis-ledger.sh

# Initialize Database
/scripts/init-db.sh

# Start Mina Archive
/scripts/start-mina-archive.sh

# Start Mina Daemon
/scripts/start-mina-daemon.sh

# Start Mina Mesh (Online)
SEARCH_TX_OPTIMIZATIONS=$MINA_MESH_SEARCH_TX_OPTIMIZATIONS MINA_MESH_PORT=$MINA_MESH_ONLINE_PORT /scripts/start-mina-mesh.sh

# Start Mina Mesh (Offline) if enabled
[[ "$MINA_MESH_OFFLINE_MODE" == "enabled" ]] && MINA_MESH_PORT=$MINA_MESH_OFFLINE_PORT /scripts/start-mina-mesh.sh

# Download Missing Blocks
/scripts/missing-blocks-guardian.sh
