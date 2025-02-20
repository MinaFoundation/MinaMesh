#!/usr/bin/env bash

set -e
set -o pipefail

# Function to download Genesis Ledger
download_genesis_ledger() {
  local url=$1
  local output_file=$2
  curl -o "$output_file" "$url"
  echo "Downloaded content from $url to $output_file"
}

echo "Fetching Genesis Ledger..."

# Download if URL is provided, else skip download
if [ -n "$MINA_GENESIS_LEDGER_URL" ]; then
  download_genesis_ledger "$MINA_GENESIS_LEDGER_URL" "$MINA_CONFIG_FILE"
else
  echo "Using local Genesis Ledger at $MINA_CONFIG_FILE"
fi
