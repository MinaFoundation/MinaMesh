#!/usr/bin/env bash

set -euxo pipefail

echo "Initializing database with Postgres $POSTGRES_VERSION..."

# Function to ensure PostgreSQL data directory exists and has correct ownership
initialize_data_dir() {
    mkdir -p "$POSTGRES_DATA_DIR"
    chown postgres:postgres "$POSTGRES_DATA_DIR"
}

# Function to initialize and start PostgreSQL
initialize_postgres() {
    sudo -u postgres "/usr/lib/postgresql/${POSTGRES_VERSION}/bin/initdb" -D "$POSTGRES_DATA_DIR" > /dev/null 2>&1
    sudo -u postgres "/usr/lib/postgresql/$POSTGRES_VERSION/bin/pg_ctl" -D "$POSTGRES_DATA_DIR" -l "${POSTGRES_DATA_DIR}/postgresql.log" start > /dev/null 2>&1
}

# Function to create PostgreSQL user and database
create_postgres_user_and_db() {
    sudo -u postgres psql --command "CREATE USER ${POSTGRES_USER} WITH SUPERUSER PASSWORD '${POSTGRES_PASSWORD}';" > /dev/null 2>&1
    sudo -u postgres createdb -O "${POSTGRES_USER}" "${POSTGRES_DBNAME}" > /dev/null 2>&1
}

# Function to find and download the latest available archive dump
download_archive_dump() {
    local MAX_DAYS_LOOKBACK=5
    local TODAY
    TODAY=$(date)
    for i in $(seq 0 $((MAX_DAYS_LOOKBACK-1))); do
        DATE=$(date -d "$TODAY - $i days" +%G-%m-%d)_${MINA_ARCHIVE_DUMP_TIME}
        local ARCHIVE_URL="${MINA_ARCHIVE_DUMP_URL}/${MINA_NETWORK}/${MINA_NETWORK}-archive-dump-${DATE}.sql.tar.gz"
        echo "Checking for dump at: $ARCHIVE_URL"

        # Check if the dump exists (HTTP status code 2xx indicates success)
        local STATUS_CODE
        STATUS_CODE=$(curl -s -o /dev/null --head -w "%{http_code}" "$ARCHIVE_URL")

        if [[ $STATUS_CODE =~ ^2[0-9]{2}$ ]]; then
            echo "Downloading ${MINA_NETWORK}-archive-dump-${DATE}..."
            curl -s "$ARCHIVE_URL" -o archive-dump.tar.gz
            return 0
        fi
    done

    echo "[WARN] Unable to find archive dump for ${MINA_NETWORK}"
    return 1
}

# Function to extract and restore the dump
restore_dump() {
    if [ -f archive-dump.tar.gz ]; then
        tar -xvf archive-dump.tar.gz
        psql -f "${MINA_NETWORK}-archive-dump-${DATE}.sql" "$POSTGRES_CONNECTION_STRING"
        rm -f archive-dump.tar.gz
    fi
}

initialize_schema() {
    echo "Initializing database schema..."
    psql "${POSTGRES_CONNECTION_STRING}" -f "${MINA_ARCHIVE_SQL_SCHEMA_PATH}"
}

# Function to show top 10 blocks in populated archiveDB
show_top_blocks() {
    echo "Top 10 blocks in populated archiveDB:"
    psql "${POSTGRES_CONNECTION_STRING}" -c "SELECT state_hash,height FROM blocks ORDER BY height DESC LIMIT 10"
}

# Main execution
initialize_data_dir
initialize_postgres
create_postgres_user_and_db
if download_archive_dump; then
    restore_dump
else
    initialize_schema
fi
show_top_blocks
