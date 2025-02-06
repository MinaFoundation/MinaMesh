# Define variables
set dotenv-required

DB_CONTAINER_NAME := "mina-mesh-db"

# Download archive DB dump
dl network:
    ./tasks/dl.sh --network {{network}}

# Initialize PostgreSQL database
pg-init:
    docker run -d --name {{DB_CONTAINER_NAME}} -p 5432:5432 -v $(pwd)/sql_scripts:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -e POSTGRES_USER=$POSTGRES_USER postgres postgres -c log_statement=all -c log_min_duration_statement=0

# Wait for PostgreSQL to be ready
pg-wait:
    ./tasks/pg_wait.sh $DATABASE_URL

# Apply optimizations to PostgreSQL
pg-apply-optimizations:
    sqlx migrate run --source sql/migrations

# Drop optimizations from PostgreSQL
pg-drop-optimizations:
    sqlx migrate revert --source sql/migrations

# Start PostgreSQL container
pg-up:
    docker start {{DB_CONTAINER_NAME}}

# Stop PostgreSQL container
pg-down:
    docker kill {{DB_CONTAINER_NAME}}

# Remove PostgreSQL container
pg-rm:
    docker rm {{DB_CONTAINER_NAME}}

# Initialize development environment
dev-init: (dl "devnet") pg-init pg-wait pg-apply-optimizations

# Run the development server
dev:
    cargo run serve --playground
