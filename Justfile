codegen:
  openapi-generator generate \
    --input-spec mesh-specifications/api.yaml \
    --generator-name=rust \
    --additional-properties=packageName=mesh \
    --output=mesh-generated \

schemas:
  mkdir -p schemas
  curl -o schemas/mina_introspection.json https://raw.githubusercontent.com/MinaProtocol/mina/develop/graphql_schema.json
  curl -o schemas/archive.graphql https://raw.githubusercontent.com/o1-labs/Archive-Node-API/main/schema.graphql
