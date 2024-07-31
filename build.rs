use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
  cynic_codegen::register_schema("archive")
    .from_sdl_file("graphql/schemas/archive.graphql")
    .unwrap()
    .as_default()
    .unwrap();
  let mut all_generated = Vec::<String>::new();
  let schema = std::fs::read_to_string("graphql/schemas/archive.graphql").unwrap();
  for entry in WalkDir::new("graphql/documents").into_iter().filter_map(Result::ok) {
    let path = entry.path();
    if path.is_file() && path.extension().map_or(false, |ext| ext == "graphql") {
      let mut file = fs::File::open(&path)?;
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      let generated = cynic_querygen::document_to_fragment_structs(
        contents,
        schema.clone(),
        &cynic_querygen::QueryGenOptions::default(),
      )
      .unwrap();
      all_generated.push(generated);
    }
  }
  let mut output_file = File::create("graphql_generated.rs").unwrap();
  output_file.write_all(all_generated.join("\n").as_bytes()).unwrap();
  Ok(())
}
