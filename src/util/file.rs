use std::path::PathBuf;
use clap::Args;

#[derive(Args, Debug)]
pub struct OutputFile {
    /// Output file
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}