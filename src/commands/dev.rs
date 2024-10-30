use std::{
  env::current_dir,
  fmt, fs,
  io::{BufRead as _, BufReader, Cursor},
  process::Command,
};

use anyhow::Result;
use chrono::{Duration, Utc};
use clap::{Args, ValueEnum};
use convert_case::{Case, Casing};
use flate2::read::GzDecoder;
use reqwest::get;
use tar::Archive;

use super::ServeCommand;
use crate::util::Docker;

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

    // Warn if gitignore exists and tmp dir not listed in it.
    let mut gitignore_path = current_dir()?.clone();
    gitignore_path.push(".gitignore");
    if gitignore_path.exists() {
      let gitignored = fs::read_to_string(gitignore_path)?.trim().split("\n").any(|s| s.contains(dev_tmp_dir));
      if !gitignored {
        println!("Warning: \"{dev_tmp_dir}\" is not yet gitignored.");
      }
    }

    let network_key = dev_network.to_string().to_case(Case::Lower);

    let mut dest = current_dir()?;
    dest.push(dev_tmp_dir);
    dest.push(&network_key);
    if dest.exists() {
      let date = (Utc::now().date_naive() - Duration::days(3)).format("%Y-%m-%d").to_string();
      let dump_url = format!("{MINA_ARCHIVE_DUMP_URL}/{network_key}-archive-dump-{date}_{dev_dump_time}.sql.tar.gz");
      let bytes = get(dump_url).await?.bytes().await?;
      let tar_gz = GzDecoder::new(Cursor::new(bytes));
      let mut archive = Archive::new(tar_gz);
      archive.unpack(dest)?;
    }

    Docker::spawn(dev_tmp_dir, &network_key);

    serve.run().await
  }
}
