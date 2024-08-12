use std::fs::File;
use std::fs::{self};
use std::io::{self};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const SCHEMAS_DIR: &str = "graphql/schemas";
const DEST_DIR: &str = "graphql_generated";

fn main() -> io::Result<()> {
  cynic_codegen::register_schema("mina")
    .from_sdl_file("graphql/schemas/mina.graphql")
    .unwrap()
    .as_default()
    .unwrap();
  cynic_codegen::register_schema("archive")
    .from_sdl_file("graphql/schemas/archive.graphql")
    .unwrap()
    .as_default()
    .unwrap();

  let mina_schema = std::fs::read_to_string(format!("{SCHEMAS_DIR}/mina.graphql"))?;
  let archive_schema = std::fs::read_to_string(format!("{SCHEMAS_DIR}/archive.graphql"))?;

  let dest_dir = Path::new(DEST_DIR);
  empty_dir(dest_dir)?;

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
  let (mut mina_documents, mut archive_documents) = (Vec::<String>::new(), Vec::<String>::new());
  for document_path in document_paths {
    let mut file = fs::File::open(&document_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if let Some(stem_os) = document_path.file_stem() {
      if let Some(stem) = stem_os.to_str() {
        let which = if stem.starts_with("archive.") {
          &mut archive_documents
        } else {
          &mut mina_documents
        };
        which.push(contents);
      }
    }
  }

  codegen(mina_schema, mina_documents, "mina")?;
  codegen(archive_schema, archive_documents, "archive")?;

  let mut mod_file = File::create(format!("{DEST_DIR}/mod.rs"))?;
  mod_file.write_all("pub mod archive;\npub mod mina;".as_bytes())?;
  Ok(())
}

fn codegen(schema: String, document_contents: Vec<String>, file_name: &str) -> io::Result<()> {
  let mut code = "".to_string();
  code.push_str("#[cynic::schema(\"");
  code.push_str(file_name);
  code.push_str("\")]");
  code.push_str("\nmod schema {}\n\n");
  if !document_contents.is_empty() {
    code.push_str(
      cynic_querygen::document_to_fragment_structs(
        document_contents.join("\n\n"),
        schema,
        &cynic_querygen::QueryGenOptions::default(),
      )
      .unwrap()
      .as_str(),
    );
  }
  let mut file = File::create(format!("{DEST_DIR}/{file_name}.rs")).unwrap();
  file.write_all(code.as_bytes())
}

fn empty_dir(dir: &Path) -> io::Result<()> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let path = entry?.path();
      if path.is_dir() {
        fs::remove_dir_all(&path)?;
      } else {
        fs::remove_file(&path)?;
      }
    }
  } else {
    fs::create_dir_all(dir)?;
  }
  Ok(())
}
