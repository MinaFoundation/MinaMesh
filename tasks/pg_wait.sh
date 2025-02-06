#!/usr/bin/env bash

DATABASE_URL=$1

# Ensure DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
  echo "DATABASE_URL is not set."
  exit 1
fi

echo "DATABASE_URL: $DATABASE_URL"

MAX_RETRIES=200
RETRY_INTERVAL_MS=10

connected=false
attempts=0

while [ "$connected" = false ] && [ $attempts -lt $MAX_RETRIES ]; do
  psql "$DATABASE_URL" -c '\q' 2>/dev/null
  if [ $? -eq 0 ]; then
    connected=true
  else
    attempts=$((attempts + 1))
    echo "Attempt $attempts failed. Waiting for database to be ready..."

    if [ $attempts -ge $MAX_RETRIES ]; then
      echo "Max retries reached. Could not connect to the database."
      exit 1
    fi

    sleep $RETRY_INTERVAL_MS
  fi
done

if [ "$connected" = true ]; then
  echo "Database ready at $DATABASE_URL"
else
  exit 1
fi
