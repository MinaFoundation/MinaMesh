use std::{
  fs,
  io::{BufRead, BufReader},
  process::Command,
};

use anyhow::Result;

pub struct Docker;

impl Docker {
  pub fn spawn(dev_tmp_dir: &str, network_key: &str) -> Result<()> {
    Self::ensure_running();
    let mut db_process = Command::new("docker")
      .args([
        "--name",
        "mina-archive-db",
        "-p",
        "5432:5432",
        "-v",
        format!("$(pwd)/${dev_tmp_dir}/{network_key}:/docker-entrypoint-initdb.d").as_str(),
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
    Ok(())
  }

  pub fn ensure_running() {
    if !Self::is_docker_installed() {
      panic!("Must install docker to use the dev command.")
    }
    if !Self::is_docker_running() {
      Self::start_docker();
    }
  }

  fn is_docker_installed() -> bool {
    match Command::new("docker").arg("--version").output() {
      Ok(output) => output.status.success(),
      Err(_) => false,
    }
  }

  fn is_docker_running() -> bool {
    match Command::new("docker").arg("info").output() {
      Ok(output) => output.status.success(),
      Err(_) => false,
    }
  }

  fn start_docker() {
    println!("Starting docker.");
    let result = Command::new("sudo").arg("systemctl").arg("start").arg("docker").status();
    match result {
      Ok(status) if status.success() => {
        println!("Docker started successfully.");
      }
      Ok(_) => {
        eprintln!("Failed to start Docker. Please check permissions or if Docker is installed.");
      }
      Err(e) => {
        eprintln!("Error starting Docker: {}", e);
      }
    }
  }
}
