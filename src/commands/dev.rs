use std::{
  env::current_dir,
  fmt,
  fs::{write, OpenOptions},
  io::{BufRead as _, BufReader, Cursor, Write},
  process::Command,
};

use anyhow::{bail, Result};
use chrono::{Duration, Utc};
use clap::{Arg, Args, ValueEnum};
use convert_case::{Case, Casing};
use cynic::{http::ReqwestExt, QueryBuilder};
use flate2::read::GzDecoder;
use reqwest::get;
use tar::Archive;

use super::ServeCommand;
use crate::util::docker::ensure_docker;

#[derive(Debug, Args)]
#[command(about = "Retrieve the genesis block identifier via a proxy node GraphQL endpoint.")]
pub struct DevCommand {
  #[arg(long, env = "MINAMESH_DEV_NETWORK", default_value_t = default_network())]
  dev_network: Network,
  #[arg(long, env = "MINAMESH_DEV_DUMP_TIME", default_value = "0000")]
  dev_dump_time: String,
  #[arg(long, env = "MINAMESH_DEV_TMP_DIR", default_value = ".minamesh")]
  dev_tmp_dir: String,
  #[command(flatten)]
  serve: ServeCommand,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Network {
  Devnet,
  Mainnet,
}

impl fmt::Display for Network {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Network::Devnet => write!(f, "devnet"),
      Network::Mainnet => write!(f, "mainnet"),
    }
  }
}

fn default_network() -> Network {
  Network::Devnet
}

const MINA_ARCHIVE_DUMP_URL: &str = "https://storage.googleapis.com/mina-archive-dumps";

impl DevCommand {
  pub async fn run(self) -> Result<()> {
    let Self { dev_dump_time, dev_network, ref dev_tmp_dir, serve } = self;

    // TODO: decide whether to implicitly write to `.gitignore`
    //
    // let mut gitignore_path = current_dir()?;
    // gitignore_path.push(".gitignore");
    // if let Ok(mut file) =
    // OpenOptions::new().create(false).append(true).open(gitignore_path) {
    //   file.write_all(dev_tmp_dir.to_string().as_bytes())?;
    // }

    let mut dest = current_dir()?;
    dest.push(dev_tmp_dir);
    if dest.exists() {
      let network_key = dev_network.to_string().to_case(Case::Lower);
      let date = (Utc::now().date_naive() - Duration::days(3)).format("%Y-%m-%d").to_string();
      let dump_url = format!("{MINA_ARCHIVE_DUMP_URL}/{network_key}-archive-dump-{date}_{dev_dump_time}.sql.tar.gz");
      let bytes = get(dump_url).await?.bytes().await?;
      let tar_gz = GzDecoder::new(Cursor::new(bytes));
      let mut archive = Archive::new(tar_gz);
      archive.unpack(dest)?;
    }
    ensure_docker();
    let mut db_process = Command::new("docker")
      .args([
        "--name",
        "mina-archive-db",
        "-p",
        "5432:5432",
        "-v",
        format!("$(pwd)/${dev_tmp_dir}:/docker-entrypoint-initdb.d").as_str(),
        "-e",
        "POSTGRES_PASSWORD=whatever",
        "-e",
        "POSTGRES_USER=mina",
        "postgres",
      ])
      .spawn()?;
    if let Some(stdout) = db_process.stdout.take() {
      let reader = BufReader::new(stdout);
      for line in reader.lines() {
        let line = line?;
        if line.contains("database system is ready to accept connections") {
          break;
        }
      }
    }
    serve.run().await
  }
}
