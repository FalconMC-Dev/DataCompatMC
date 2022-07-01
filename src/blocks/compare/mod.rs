use clap::Args;
use crate::util::file::InputFile;

#[derive(Args, Debug)]
/// Compare two data versions one-directional
///
/// Currently only compares block data
pub struct CompareCommand {
    /// The intermediary data file to start from
    base: InputFile,
    /// The target intermediary data file to find compatibility with
    target: InputFile,
}

impl CompareCommand {
    pub fn compare(&self) -> anyhow::Result<()> {
        eprintln!("Comparing...");
        Ok(())
    }
}