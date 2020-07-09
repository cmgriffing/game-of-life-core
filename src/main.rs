mod cli;
mod core;

use crate::cli::render_seeds::render_seeds;
use crate::cli::validate_submissions::validate_submissions;
use log::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
  command: String,
  commandArgument: Option<String>,
}

fn main() {
  let args = Cli::from_args();
  let command: &str = &args.command;
  println!("Running command: {:?}", command);
  match command {
    "validate" => validate_submissions(),
    "render_seeds" => render_seeds(),
    _ => warn!("Command: {:?} not found", args.command),
  };
}
