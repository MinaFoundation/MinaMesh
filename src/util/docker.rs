use std::process::Command;

pub fn ensure_docker() {
  if !is_docker_installed() {
    panic!("Must install docker to use the dev command.")
  }
  if !is_docker_running() {
    start_docker();
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
