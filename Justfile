pg:
  docker run -d --name mina-archive-db -p 5432:5432 -v $(pwd)/sql_scripts:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=whatever -e POSTGRES_USER=mina postgres

pg-up:
  docker start mina-archive-db

pg-down:
  docker kill mina-archive-db

pg-rm:
  docker rm mina-archive-db

get-mainnet-archive-db:
  ./scripts/get_archive_db.sh mainnet

wait-for-pg:
  ./scripts/wait_for_pg.sh

test:
  SNAP_CHECK=1 cargo test