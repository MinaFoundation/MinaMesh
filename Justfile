spec_path := "mesh-specifications/api.yaml"
graphql_schema_dest := "graphql_introspection_schema.json"

codegen:
  openapi-generator generate \
    --input-spec {{spec_path}} \
    --generator-name=rust \
    --output=mesh-generated \
    --additional-properties=packageName=mesh

download-graphql-schema:
  curl -o {{graphql_schema_dest}} https://raw.githubusercontent.com/MinaProtocol/mina/develop/graphql_schema.json
