use std::{fs, io::Write, path::PathBuf};

use anyhow::Result;

fn main() -> Result<()> {
  // TODO: why is the following line not working?
  // println!("cargo:rerun-if-changed=./src/graphql");
  let mina_schema = std::fs::read_to_string("src/graphql/schema/mina_schema.graphql")?;
  cynic_codegen::register_schema("mina").from_sdl(mina_schema.as_str())?.as_default()?;
  let document_paths = fs::read_dir("src/graphql")?
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
    .collect::<Vec<PathBuf>>();
  let mut code = indoc::indoc! { r#"
    // This file is generated by `build.rs`. Do not edit it directly.
    #![allow(dead_code)]

    #[cynic::schema("mina")]
    mod schema {}

  "# }
  .to_string();
  let document_contents =
    document_paths.iter().map(std::fs::read_to_string).collect::<Result<Vec<String>, _>>()?.join("\n\n");
  code.push_str(
    cynic_querygen::document_to_fragment_structs(document_contents, mina_schema, &cynic_querygen::QueryGenOptions {
      schema_module_name: "mina".to_string(),
      schema_name: Some("mina".to_string()),
    })?
    .as_str(),
  );
  let mut mod_file = fs::File::create("src/graphql/generated.rs")?;
  mod_file.write_all(code.as_bytes())?;
  Ok(())
}