use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

const SCHEMA_PATH: &str = "schema/mina_schema.graphql";

fn main() -> io::Result<()> {
  let mina_schema = std::fs::read_to_string(SCHEMA_PATH)?;
  cynic_codegen::register_schema("mina")
    .from_sdl(mina_schema.as_str())
    .unwrap()
    .as_default()
    .unwrap();
  let document_paths: Vec<PathBuf> = fs::read_dir(".")?
    .filter_map(|entry_result| match entry_result {
      Ok(entry) => {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "graphql") {
          Some(entry_path)
        } else {
          None
        }
      }
      _ => None,
    })
    .collect();
  let mut code = "#[cynic::schema(\"mina\")]\nmod schema {}\n\n".to_string();
  let mut document_contents = Vec::<String>::new();
  for document_path in document_paths {
    document_contents.push(std::fs::read_to_string(document_path)?);
  }
  code.push_str(
    cynic_querygen::document_to_fragment_structs(
      document_contents.join("\n\n"),
      mina_schema,
      &cynic_querygen::QueryGenOptions {
        schema_module_name: "mina".to_string(),
        schema_name: Some("mina".to_string()),
      },
    )
    .unwrap()
    .as_str(),
  );
  let mut mod_file = File::create("generated.rs")?;
  mod_file.write_all(code.as_bytes())?;
  Ok(())
}
