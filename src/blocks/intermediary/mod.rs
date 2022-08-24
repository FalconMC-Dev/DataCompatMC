use clap::Args;
use anyhow::Result;
use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use crate::blocks::intermediary::data::ModernBlockList;
use crate::blocks::intermediary::rules::ModernPropertyRules;

use crate::blocks::raw::de::CompactRuleProvider;
use crate::util::file::{InputFile, OutputFile};

use self::collisions::{CollisionRuleProvider, CollisionList};

pub mod collisions;
pub mod data;
pub mod rules;

#[derive(Args, Debug)]
/// Generates intermediate data
///
/// Fed with a raw data file from the Minecraft generators, this command will
/// generate a more compact version of the same data, applying tricks to
/// minimize the size in a lossless manner. To avoid property collisions between
/// blocks, a rules file can be specified (property collisions fail!).
pub struct IntermediaryCommand {
    /// File containing raw generated data from Minecraft
    input: InputFile,
    /// A file specifying rules on block properties
    #[clap(short, long)]
    rules: Option<InputFile>,
    #[clap(short, long)]
    output: Option<OutputFile>,
    #[clap(long = "no-pretty")]
    /// Does not pretty-print the resulting json data
    no_pretty: bool,
}

impl IntermediaryCommand {
    pub fn generate_intermediate(&self) -> Result<()> {
        let data = self.input.data();

        let rules: Option<ModernPropertyRules> = self.rules
            .as_ref()
            .map(|rules| rules.deserialized()).transpose()?;

        eprintln!("Checking for property collisions...");
        let collisions = CollisionRuleProvider::new(rules.as_ref());
        let collisions: CollisionList = collisions.deserialize(&mut Deserializer::from_str(data))?; 

        collisions.display();
        if collisions.should_exit() {
            eprintln!("Could not continue due to one or more collisions in block properties.\nPlease specify a rules file to resolve these");
            return Ok(())
        }
        eprintln!("No serious collisions found, slight inefficiencies will have been signaled by now. \u{2705}");

        let compacter = CompactRuleProvider::new(rules.as_ref());
        let modern_data: ModernBlockList = compacter.deserialize(&mut Deserializer::from_str(data))?;

        match &self.output {
            Some(output) => {
                if let Some(writer) = output.writer()? {
                    if self.no_pretty {
                        serde_json::to_writer(writer, &modern_data)?;
                    } else {
                        serde_json::to_writer_pretty(writer, &modern_data)?;
                    }
                    eprintln!("Successfully compacted data \u{2705}");
                } else {
                    eprintln!("Aborted");
                }
            }
            None => {
                let result = if self.no_pretty {
                    serde_json::to_string(&modern_data)?
                } else {
                    serde_json::to_string_pretty(&modern_data)?
                };
                println!("{}", result);
                eprintln!("========\nSuccessfully compacted data \u{2705}");
            }
        }

        Ok(())
        // let mut data: RawBlockList = self.input.data()?;
        // eprintln!("Loaded {} blocks \u{2705}\nLoaded {} properties \u{2705}", data.blocks().len(), data.properties().len());
        // if let Some(rules) = &self.rules {
        //     eprintln!("Applying property rules...");
        //     let rules: ModernPropertyRules = rules.data()?;
        //     rules.apply_enum_rules(&mut data);
        // }
        // if !self.check_collisions(data.properties_mut()) {
        //     eprintln!("\nCould not continue due to one or more collisions in block properties.\nPlease specify a rules file to resolve the collision(s).");
        //     return Ok(());
        // }
        //
        // let modern_data = ModernBlockList::from(data);
        // match &self.output {
        //     Some(output) => {
        //         if let Some(writer) = output.writer()? {
        //             if self.no_pretty {
        //                 serde_json::to_writer(writer, &modern_data)?;
        //             } else {
        //                 serde_json::to_writer_pretty(writer, &modern_data)?;
        //             }
        //             eprintln!("Successfully compacted data \u{2705}");
        //         } else {
        //             eprintln!("Aborted");
        //         }
        //     }
        //     None => {
        //         let result = if self.no_pretty {
        //             serde_json::to_string(&modern_data)?
        //         } else {
        //             serde_json::to_string_pretty(&modern_data)?
        //         };
        //         println!("{}", result);
        //         eprintln!("========\nSuccessfully compacted data \u{2705}");
        //     }
        // }
        // Ok(())
    }
}
