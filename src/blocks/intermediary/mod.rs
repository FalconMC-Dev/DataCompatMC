use std::path::PathBuf;

use clap::Args;
use anyhow::Result;

use crate::util::file::OutputFile;

#[derive(Args, Debug)]
/// Generates intermediate data
///
/// Fed with a raw data file from the Minecraft generators, this command will
/// generate a more compact version of the same data, applying tricks to
/// minimize the size in a lossless manner. To avoid property collisions between
/// blocks, a rules file can be specified (property collisions fail!).
pub struct IntermediaryCommand {
    /// Input file
    input: PathBuf,
    #[clap(flatten)]
    output: OutputFile,
}

impl IntermediaryCommand {
    pub fn generate_intermediate(&self) -> Result<()> {

        Ok(())
    }
}
