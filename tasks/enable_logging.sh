#!/bin/bash

# This script enables logging of sql statements in the PostgreSQL database
# Put this script in the `../sql_scripts` directory and once you use `deno task pg:init` or `deno task dev:init` it will be executed automatically
# In order to follow the logs, you can use `docker mina-archive-db logs -f`

echo "Enabling logging settings..."

echo "log_statement = 'all'" >> /var/lib/postgresql/data/postgresql.conf
echo "log_min_duration_statement = 0" >> /var/lib/postgresql/data/postgresql.conf
