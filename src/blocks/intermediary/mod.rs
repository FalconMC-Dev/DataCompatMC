use clap::Args;
use anyhow::Result;
use crate::blocks::intermediary::data::ModernBlockList;

use crate::util::file::{InputFile, OutputFile};

pub mod data;

#[derive(Args, Debug)]
/// Generates intermediate data
///
/// Fed with a raw data file from the Minecraft generators, this command will
/// generate a more compact version of the same data, applying tricks to
/// minimize the size in a lossless manner. To avoid property collisions between
/// blocks, a rules file can be specified (property collisions fail!).
pub struct IntermediaryCommand {
    #[clap(flatten)]
    input: InputFile,
    #[clap(flatten)]
    output: OutputFile,
}

impl IntermediaryCommand {
    pub fn generate_intermediate(&self) -> Result<()> {
        let data: ModernBlockList = self.input.data()?;
        /*if let Some(data) = data.iter().nth(4) {
            println!("data: {:?}", data);
        }*/
        eprintln!("found {} blocks and {} properties!", data.blocks.len(), data.properties.len());
        Ok(())
    }
}
