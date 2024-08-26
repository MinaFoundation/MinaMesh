use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

const SCHEMA_PATH: &str = "schema.graphql";
const DEST_PATH: &str = "graphql_generated.rs";

fn main() -> io::Result<()> {
  let mina_schema = std::fs::read_to_string(SCHEMA_PATH)?;
  cynic_codegen::register_schema("mina")
    .from_sdl(mina_schema.as_str())
    .unwrap()
    .as_default()
    .unwrap();
  fs::remove_file(DEST_PATH)?;
  let document_paths: Vec<PathBuf> = fs::read_dir("graphql")?
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
  let mut file = fs::File::open(DEST_PATH)?;
  let mut code = "#[cynic::schema(\"mina\")]\nmod schema {{}}\n\n".to_string();
  file.read_to_string(&mut code);
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
  let mut mod_file = File::create(DEST_PATH)?;
  mod_file.write_all("pub mod mina;".as_bytes())?;
  Ok(())
}

fn codegen(schema: String, document_contents: Vec<String>, file_name: &str) -> io::Result<()> {
  if !document_contents.is_empty() {
    code.push_str(
      cynic_querygen::document_to_fragment_structs(
        document_contents.join("\n\n"),
        schema,
        &cynic_querygen::QueryGenOptions {
          schema_module_name: file_name.to_string(),
          schema_name: Some(file_name.to_string()),
        },
      )
      .unwrap()
      .as_str(),
    );
  }
  let mut file = File::create(format!("{DEST_DIR}/{file_name}.rs")).unwrap();
  file.write_all(code.as_bytes())
}
