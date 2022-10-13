use anyhow::Context;
use clap::{Parser, Subcommand};
use cmd::{InfoCommand, IntermediaryCommand};

pub mod blocks;
pub mod cmd;
pub mod util;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    Intermediary(IntermediaryCommand),
    Info(InfoCommand),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Intermediary(cmd) => cmd.generate_intermediate().context("Error while generating data"),
        SubCommands::Info(cmd) => cmd.display_info().context("Error while displaying info"),
    }
}
