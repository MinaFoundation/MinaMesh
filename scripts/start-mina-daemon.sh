#!/usr/bin/env bash

set -e
set -o pipefail

# Function to log environment variables
log_environment() {
    echo "Starting Mina Daemon with the following configurations:"
    echo "  Peer List URL: $PEER_LIST_URL"
    echo "  Mina GraphQL Port: $MINA_GRAPHQL_PORT"
    echo "  Mina Archive Port: $MINA_ARCHIVE_PORT"
    echo "  Log Level: $MINA_DAEMON_LOG_LEVEL"
}

# Function to start Mina Mesh server
start_mina_daemon() {
    mina daemon \
      --peer-list-url "${PEER_LIST_URL}" \
      --rest-port "${MINA_GRAPHQL_PORT}" \
      -archive-address "127.0.0.1:${MINA_ARCHIVE_PORT}" \
      -insecure-rest-server \
      --log-level "${MINA_DAEMON_LOG_LEVEL}" \
      --log-json \
      "$@" &
    export MINA_DAEMON_PID=$!  # Storing the PID of the daemon process
}

# Function to check mina client status for liveness
check_liveness() {
  mina client status --json >/dev/null 2>&1
  return $?
}

wait_for_graphql() {
    echo "Waiting for Mina GraphQL endpoint to be ready..."
    local retries=30  # Maximum number of retries
    local delay=60    # Delay in seconds between retries

    for ((i=1; i<=retries; i++)); do
        if check_liveness; then
            echo "Mina GraphQL endpoint is up!"
            return 0
        fi
        echo "Attempt $i/$retries: GraphQL not ready yet. Retrying in $delay seconds..."
        sleep $delay
    done

    echo "Error: Mina GraphQL endpoint did not become ready in time. Exiting."
    exit 1
}


# Main execution
log_environment
start_mina_daemon "${MINA_EXTRA_FLAGS[@]}"
wait_for_graphql
