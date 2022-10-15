use anyhow::Result;
use clap::Args;

use crate::blocks::intermediary::data::{ModernBlockList, ModernBlockData, PropertyValue};
use crate::util::file::InputFile;

#[derive(Args, Debug)]
pub struct InfoCommand {
    /// File containing intermediary data from mc-data
    input: InputFile,
}

impl InfoCommand {
    pub fn display_info(&self) -> Result<()> {
        let data: ModernBlockList = self.input.deserialized()?;

        if let Some(meta) = data.metadata {
            if let Some(name) = meta.name {
                println!("Minecraft blockdata version {} ({})", meta.id, name);
            } else {
                println!("Minecraft blockdata version {}", meta.id);
            }

            if let Some(note) = meta.note {
                println!("Note: {}", note);
            }
        } else {
            println!("Minecraft blockdata version UNKNOWN");
        }

        println!("Loaded {} blocks successfully \u{2705}", data.blocks.len());
        println!("There are {} enum properties present \u{2705}\n------", data.properties.len());

        // healthcheck
        let mut blocks: Vec<ModernBlockData> = data.blocks.into_iter().map(|(_, value)| value).collect();
        blocks.sort_by_key(|data| data.base_id);

        let mut unknown = Vec::new();
        let mut missing = Vec::new();
        let mut counter = 0;
        for block in blocks {
            if counter != block.base_id {
                let difference = block.base_id - counter;
                if difference == 1 {
                    missing.push(format!("{}", counter));
                } else {
                    missing.push(format!("{}..{}", counter, block.base_id));
                }
                counter = block.base_id;
            }

            let mut state_count = 1;
            for (_, property) in &block.kinds {
                match property {
                    PropertyValue::Range(range) => state_count *= (range[1] - range[0] + 1) as usize,
                    PropertyValue::Text(text) => {
                        if text == &"bool" {
                            state_count *= 2;
                        } else {
                            match data.properties.get(text) {
                                Some(property) => state_count *= property.fields().len(),
                                None => unknown.push(text.to_string()),
                            }
                        }
                    }
                }
            }
            counter += state_count as i32;
        }

        if unknown.is_empty() && missing.is_empty() {
            println!("Healthcheck success! \u{2705}");
        } else {
            println!("Healthcheck failed: \u{274C}");
            if !unknown.is_empty() {
                println!("The follow properties could not be found: \u{274C}");
                unknown.into_iter().fold(true, |first, elem| {
                    if !first {
                        print!(", {}", elem);
                    } else {
                        print!("{}", elem);
                    }
                    false
                });
                println!();
            }
            if !missing.is_empty() {
                println!("The following blockstates could not be found: \u{274C}");
                missing.into_iter().fold(true, |first, elem| {
                    if !first {
                        print!(", {}", elem);
                    } else {
                        print!("{}", elem);
                    }
                    false
                });
                println!();
            }
        }

        Ok(())
    }
}
