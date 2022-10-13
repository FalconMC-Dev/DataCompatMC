use anyhow::Result;
use clap::Args;

use crate::blocks::intermediary::data::ModernBlockList;
use crate::util::file::InputFile;

#[derive(Args, Debug)]
pub struct InfoCommand {
    /// File containing intermediary data from mc-data
    input: InputFile,
}

impl InfoCommand {
    pub fn display_info(&self) -> Result<()> {
        let data: ModernBlockList = self.input.deserialized()?;

        let version;
        let note;
        if let Some(meta) = data.metadata {
            let id = meta.id;
            version = if let Some(name) = meta.name {
                format!("Minecraft blockdata version {id} ({name})\n")
            } else {
                format!("Minecraft blockdata version {id}\n")
            };

            note = if let Some(x) = meta.note {
                format!("Note: {x}\n")
            } else {
                String::from("")
            };
        } else {
            version = String::from("");
            note = String::from("");
        };

        let blocks_len = data.blocks.len();
        let properties_len = data.properties.len();

        println!(
            "{version}{note}
Loaded {blocks_len} blocks successfully
There are {properties_len} enum properties present"
        );

        Ok(())
    }
}
