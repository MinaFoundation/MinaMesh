pg:
  docker run -d --name mina-archive-db -p 5432:5432 -v $(pwd)/sql_scripts:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=whatever -e POSTGRES_USER=mina postgres
