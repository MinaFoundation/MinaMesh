#!/bin/bash

# This script is used to download the archive dump from the mina-archive-dumps bucket
# and extract the archive dump to the sql_scripts directory
# The script will download the archive dump for the last 5 days and extract the first available archive dump
# Usage: ./scripts/get_archive_db.sh <MINA_NETWORK>
# Example: ./scripts/get_archive_db.sh mainnet

MINA_NETWORK=${1}
MINA_ARCHIVE_DUMP_URL=${MINA_ARCHIVE_DUMP_URL:=https://storage.googleapis.com/mina-archive-dumps}
DUMP_TIME=0000
SQL_SCRIPT_PATH=$(pwd)/sql_scripts
TAR_FILE_PATH=${SQL_SCRIPT_PATH}/o1labs-archive-dump.tar.gz

mkdir -p ${SQL_SCRIPT_PATH}

MAX_DAYS_LOOKBACK=5
i=0
while [ $i -lt $MAX_DAYS_LOOKBACK ]; do
    DATE=$(date -d "$i days ago" +%G-%m-%d)_${DUMP_TIME}
    STATUS_CODE=$(curl -s -o /dev/null --head -w "%{http_code}" "${MINA_ARCHIVE_DUMP_URL}/${MINA_NETWORK}-archive-dump-${DATE}.sql.tar.gz")
    if [[ ! $STATUS_CODE =~ 2[0-9]{2} ]]; then
        i=$((i + 1))
    else
        echo "Download ${MINA_NETWORK}-archive-dump-${DATE}.sql.tar.gz"
        curl "${MINA_ARCHIVE_DUMP_URL}/${MINA_NETWORK}-archive-dump-${DATE}.sql.tar.gz" -o ${TAR_FILE_PATH}
        break
    fi
done

[[ $STATUS_CODE =~ 2[0-9]{2} ]] || echo "[WARN] Unable to find archive dump for ${MINA_NETWORK}"

tar -xvf ${SQL_SCRIPT_PATH}/o1labs-archive-dump.tar.gz -C ${SQL_SCRIPT_PATH}
rm -f ${TAR_FILE_PATH}

echo "Extracted ${MINA_NETWORK}-archive-dump-${DATE}.sql.tar.gz to ${SQL_SCRIPT_PATH}/${MINA_NETWORK}-archive-dump-${DATE}.sql"
