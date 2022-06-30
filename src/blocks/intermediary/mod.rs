use clap::Args;
use anyhow::Result;
use crate::blocks::intermediary::data::ModernBlockList;
use crate::blocks::intermediary::rules::ModernPropertyRules;
use crate::blocks::raw::modern::{PropertyKind, RawBlockList};

use crate::util::file::{InputFile, OutputFile};

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
        let mut data: RawBlockList = self.input.data()?;
        eprintln!("Loaded {} blocks \u{2705}\nLoaded {} properties \u{2705}", data.blocks().len(), data.properties().len());
        if let Some(rules) = &self.rules {
            eprintln!("Applying property rules...");
            let rules: ModernPropertyRules = rules.data()?;
            rules.apply_enum_rules(&mut data);
        }
        if !self.check_collisions(data.properties_mut()) {
            eprintln!("\nCould not continue due to one or more collisions in block properties.\nPlease specify a rules file to resolve the collision(s).");
            return Ok(());
        }

        let modern_data = ModernBlockList::from(data);
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
    }
    
    fn check_collisions<'raw>(&self, properties: &mut Vec<(&'raw str, PropertyKind<'raw>)>) -> bool {
        eprintln!("Checking for property collisions...\n========");
        properties.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));
        let mut iterator = properties.iter()
            .filter_map(|(n, k)| match k {
                PropertyKind::Enum(values) => Some((n, values)),
                _ => None
            }).peekable();
        let mut count = 0;
        let mut streak = false;
        while let Some((&name, property)) = iterator.next() {
            if let Some((&next_name, _)) = iterator.peek() {
                if name == next_name {
                    streak = true;
                    eprintln!("{} -> {:?}", name, property.fields());
                } else if streak {
                    streak = false;
                    count += 1;
                    eprintln!("{} -> {:?}\n========", name, property.fields());
                }
            } else {
                if streak {
                    count += 1;
                    eprintln!("{} -> {:?}\n========", name, property.fields());
                }
            }
        }
        if count > 0 {
            eprintln!("Found {} property collisions! \u{274C}", count);
            false
        } else {
            eprintln!("No property collisions found! \u{2705}\n========");
            true
        }
    }
}
