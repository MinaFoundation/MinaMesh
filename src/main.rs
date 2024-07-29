use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
  name = "mina-mesh",
  version,
  about = "A Mesh-compliant Server for Mina",
  propagate_version = true,
  author = "Mina Foundation"
)]
struct MinaMeshArgs {
  #[arg(long, short = 'v', default_value = "false", global = true)]
  verbose: bool,
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(about = "Start the Mina Mesh Server")]
  Start {
    #[arg(long, short = 'p', default_value = "8686")]
    port: u16,
  },
}

fn main() {
  let args = MinaMeshArgs::parse();
  println!("Hello, Mina Mesh!, {:?}", args);
}
