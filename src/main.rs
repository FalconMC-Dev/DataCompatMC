use clap::{Parser, Subcommand};
use blocks::intermediary::IntermediaryCommand;

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
    Intermediary(IntermediaryCommand),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Intermediary(cmd) => {
            if let Err(error) = cmd.generate_intermediate() {
                eprintln!("Error while generating data: {}", error);
            }
        }
    }
}
