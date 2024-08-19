codegen:
  openapi-generator generate \
    --input-spec mesh-specifications/api.yaml \
    --generator-name=rust \
    --additional-properties=packageName=mesh \
    --output=mesh-generated \

schemas:
  mkdir -p graphql/schemas
  curl -o graphql/schemas/mina_introspection.json https://raw.githubusercontent.com/MinaProtocol/mina/develop/graphql_schema.json
  curl -o graphql/schemas/archive.graphql https://raw.githubusercontent.com/o1-labs/Archive-Node-API/main/schema.graphql

pg:
  docker run -d --name mina-archive-db -p 5432:5432 -v $(pwd)/sql_scripts:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=whatever -e POSTGRES_USER=mina postgres
