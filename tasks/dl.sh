#!/usr/bin/env bash

# Constants
MINA_ARCHIVE_DUMP_URL="https://storage.googleapis.com/mina-archive-dumps"
DEST_DIR="$(pwd)/sql_scripts"
DEST="$DEST_DIR/archive.tar.gz"

# Default values
NETWORK="devnet"
DUMP_TIME="0000"

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --network) NETWORK="$2"; shift ;;
        --dump-time) DUMP_TIME="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Validate network
if [[ "$NETWORK" != "devnet" && "$NETWORK" != "mainnet" ]]; then
    echo "Invalid network: $NETWORK"
    exit 1
fi

# Calculate date (3 days ago)
DATE=$(date -d "-3 days" +"%Y-%m-%d")

# Construct dump URL
DUMP_URL="${MINA_ARCHIVE_DUMP_URL}/${NETWORK}-archive-dump-${DATE}_${DUMP_TIME}.sql.tar.gz"

# Print download information
echo "Downloading $DUMP_URL to $DEST"

# Create destination directory
mkdir -p "$DEST_DIR"
rm -rf "$DEST_DIR"/*

# Download and decompress the dump
curl -L "$DUMP_URL" | tar -xz -C "$DEST_DIR"

# Note: The above command uses tar to directly extract the downloaded archive.
# If you need to save the tar file first and then extract, you can use the following commands:
# curl -L "$DUMP_URL" -o "$DEST"
# tar -xf "$DEST" -C "$DEST_DIR"
