use anyhow::Context;
use clap::{Parser, Subcommand};
use blocks::intermediary::IntermediaryCommand;
use blocks::compare::CompareCommand;

mod blocks;
mod util;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    Compare(CompareCommand),
    Intermediary(IntermediaryCommand),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Intermediary(cmd) => {
            cmd.generate_intermediate().with_context(|| "Error while generating data")
        }
        SubCommands::Compare(cmd) => {
            cmd.compare().with_context(|| "Error while comparing data")
        }
    }
}
