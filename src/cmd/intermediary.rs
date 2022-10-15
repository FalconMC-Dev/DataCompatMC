use anyhow::Result;
use clap::Args;
use serde::de::DeserializeSeed;
use serde_json::Deserializer;

use crate::blocks::intermediary::MetaData;
use crate::blocks::intermediary::collisions::{CollisionList, CollisionRuleProvider};
use crate::blocks::intermediary::data::ModernBlockList;
use crate::blocks::intermediary::rules::ModernPropertyRules;
use crate::blocks::raw::de::CompactRuleProvider;
use crate::util::file::{InputFile, OutputFile};

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
    /// The ID of the minecraft version the raw data comes from (e.g 2730)
    #[clap(long)]
    id: Option<i32>,
    /// The pretty version number (e.g 1.17.1)
    #[clap(short = 'd', long, requires = "id")]
    display_name: Option<String>,
    #[clap(long, requires = "id")]
    note: Option<String>,
    #[clap(long)]
    /// Does not pretty-print the resulting json data
    no_pretty: bool,
}

impl IntermediaryCommand {
    pub fn generate_intermediate(&self) -> Result<()> {
        // Load data
        let data = self.input.data();

        // Load rules
        let rules: Option<ModernPropertyRules> = self.rules.as_ref().map(|rules| rules.deserialized()).transpose()?;

        // Property collisions
        eprintln!("Checking for property collisions...");
        let collisions = CollisionRuleProvider::new(rules.as_ref());
        let collisions: CollisionList = collisions.deserialize(&mut Deserializer::from_str(data))?;

        collisions.display();
        if collisions.should_exit() {
            eprintln!("Could not continue due to one or more collisions in block properties.\nPlease specify a rules file to resolve these");
            return Ok(());
        }
        eprintln!("No serious collisions found, slight inefficiencies will have been signaled by now. \u{2705}");

        // Compact data and print to output
        let metadata = if let Some(id) = self.id {
            Some(MetaData::new(id, self.display_name.as_deref(), self.note.as_deref()))
        } else {
            None
        };
        let compacter = CompactRuleProvider::new(rules.as_ref(), metadata);
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
            },
            None => {
                let result = if self.no_pretty {
                    serde_json::to_string(&modern_data)?
                } else {
                    serde_json::to_string_pretty(&modern_data)?
                };
                println!("{}", result);
                eprintln!("========\nSuccessfully compacted data \u{2705}");
            },
        }

        Ok(())
    }
}
