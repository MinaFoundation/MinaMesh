spec_path := "mesh-specifications/api.yaml"

codegen:
  openapi-generator generate \
    --input-spec {{spec_path}} \
    --generator-name=rust \
    --output=mesh-generated \
    --additional-properties=packageName=mesh
