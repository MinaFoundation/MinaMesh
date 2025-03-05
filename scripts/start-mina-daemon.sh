#!/usr/bin/env bash

set -e
set -o pipefail

# Function to log environment variables
log_environment() {
    echo "Starting Mina Daemon with the following configurations:"
    echo "  Mina Network: $MINA_NETWORK"
    [[ -n "$PEER_LIST_URL" ]] && echo "  Peer List URL: $PEER_LIST_URL"
    echo "  Mina GraphQL Port: $MINA_GRAPHQL_PORT"
    echo "  Mina Archive Port: $MINA_ARCHIVE_PORT"
    echo "  Log Level: $MINA_DAEMON_LOG_LEVEL"
}

# Function to start Mina daemon
start_mina_daemon() {
    local args=(
        --archive-address "127.0.0.1:${MINA_ARCHIVE_PORT}"
        --rest-port "${MINA_GRAPHQL_PORT}"
        --insecure-rest-server
    )

    if [[ "$MINA_NETWORK" == "local" ]]; then
        args+=(
            --block-producer-pubkey "$(cat "$MINA_KEYS_PATH/block-producer.key.pub")"
            --config-directory "${MINA_CONFIG_DIR}"
            --config-file "${MINA_GENESIS_LEDGER}"
            --demo-mode
            --log-level "${MINA_DAEMON_LOG_LEVEL}"
            --proof-level none
            --run-snark-worker "$(cat "$MINA_KEYS_PATH/snark-producer.key.pub")"
            --seed
        )
    else
        args+=(
            --peer-list-url "${PEER_LIST_URL}"
            --log-level "${MINA_DAEMON_LOG_LEVEL}"
        )
    fi

    if [[ -n "${MINA_EXTRA_FLAGS[*]}" ]]; then
        args+=("${MINA_EXTRA_FLAGS[@]}")
    fi
    echo "Running: mina daemon" "${args[@]}" # Debug output
    mina daemon "${args[@]}" &
    export MINA_DAEMON_PID=$!  # Store the PID of the daemon process
}

monitor_mina_daemon() {
    while true; do
        if ! kill -0 "$MINA_DAEMON_PID" 2>/dev/null; then
            echo "Mina daemon is not running. Restarting..."
            start_mina_daemon
        fi
        sleep 15
    done
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

if [[ "$MINA_NETWORK" == "local" ]]; then
    unset PEER_LIST_URL
fi

log_environment
start_mina_daemon
wait_for_graphql
monitor_mina_daemon &
