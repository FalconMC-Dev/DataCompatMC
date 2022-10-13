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

        let num_of_blocks = data.blocks.len();
        let num_of_enum_props = data.properties.len();

        println!(
            "{version}{note}
Loaded {num_of_blocks} blocks successfully
There are {num_of_enum_props} enum properties present"
        );

        Ok(())
    }
}
