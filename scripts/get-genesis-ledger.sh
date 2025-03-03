#!/usr/bin/env bash

set -euxo pipefail

# Function to download Genesis Ledger
download_genesis_ledger() {
  curl -o "$MINA_GENESIS_LEDGER" "$MINA_GENESIS_LEDGER_URL"
  echo "Downloaded content from $MINA_GENESIS_LEDGER_URL to $MINA_GENESIS_LEDGER"
}

# Function to download Genesis Ledger
generate_genesis_ledger() {
  mkdir -p "$MINA_KEYS_PATH"
  mina advanced generate-keypair --privkey-path "$MINA_KEYS_PATH/block-producer.key"
  mina advanced generate-keypair --privkey-path "$MINA_KEYS_PATH/snark-producer.key"
  chmod -R 0700 "$MINA_KEYS_PATH"
  BLOCK_PRODUCER_PK=$(cat "$MINA_KEYS_PATH/block-producer.key.pub")
  SNARK_PRODUCER_PK=$(cat "$MINA_KEYS_PATH/snark-producer.key.pub")


  mkdir -p "$MINA_CONFIG_DIR/wallets/store"
  cp "$MINA_KEYS_PATH/block-producer.key" "$MINA_CONFIG_DIR/wallets/store/$BLOCK_PRODUCER_PK"
  CURRENT_TIME=$(date +"%Y-%m-%dT%H:%M:%S%z")
  rm -f /var/lib/coda/*.json
  cat <<EOF >"$MINA_GENESIS_LEDGER"
{
  "genesis": { "genesis_state_timestamp": "$CURRENT_TIME" },
  "proof": { "block_window_duration_ms": 20000 },
  "ledger": {
    "name": "${MINA_NETWORK}",
    "accounts": [
      { "pk": "${BLOCK_PRODUCER_PK}", "balance": "10000", "delegate": null, "sk": null },
      { "pk": "${SNARK_PRODUCER_PK}", "balance": "20000", "delegate": "${BLOCK_PRODUCER_PK}", "sk": null }
    ]
  }
}
EOF
  echo "Genesis Ledger successfully generated and saved to $MINA_GENESIS_LEDGER"
}

# Download if URL is provided, else skip download
if [ -n "$MINA_GENESIS_LEDGER_URL" ]; then
  echo "Fetching Genesis Ledger..."
  download_genesis_ledger
elif [ "$MINA_NETWORK" == "local" ]; then
  echo "Generating Genesis Ledger..."
  generate_genesis_ledger
else
  echo "Using local Genesis Ledger at $MINA_GENESIS_LEDGER"
fi
