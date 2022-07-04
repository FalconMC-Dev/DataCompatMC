use anyhow::Context;
use clap::Args;
use crate::blocks::compare::kinds::{BlockComparison, PropertyComparison};
use crate::blocks::intermediary::data::ModernBlockList;
use crate::util::file::InputFile;

pub mod kinds;

#[derive(Args, Debug)]
/// Compare two data versions one-directional
///
/// Currently only compares block data
pub struct CompareCommand {
    /// The intermediary data file to start from
    base: InputFile,
    /// The target intermediary data file to find compatibility with
    target: InputFile,
    /// Don't list missing entries
    #[clap(long = "print-missing")]
    print_missing: bool,
}

impl CompareCommand {
    pub fn compare<'raw>(&self) -> anyhow::Result<()> {
        let base_version: ModernBlockList = self.base.data().with_context(|| format!("Error deserializing {:?}", self.base.name()))?;
        let target_version: ModernBlockList = self.target.data().with_context(|| format!("Error deserializing {:?}", self.target.name()))?;

        eprintln!("Checking property compatibility...\n=============");
        let mut missing_props = 0;
        for (property, base) in base_version.properties() {
            if let Some(target) = target_version.properties().get(property) {
                match PropertyComparison::compare(base, target) {
                    PropertyComparison::Equal => {}
                    cmp => println!("\"{}\" has {:?}", property, cmp),
                }
            } else {
                if self.print_missing {
                    println!("Missing property: \"{}\"!", property);
                }
                missing_props += 1;
            }
        }
        eprintln!("=============\nChecking block compatibility...\n=============");
        let mut missing_count = 0;
        let mut same_base = 0;
        let mut perfect_count = 0;
        for (name, data) in base_version.blocks() {
            if let Some(target) = target_version.blocks().get(name) {
                match BlockComparison::compare(data, target) {
                    BlockComparison::Equal => {
                        if data.base_id() == target.base_id() {
                            perfect_count += 1;
                        }
                    }
                    cmp => eprintln!("\"{}\" has \"{:?}\"", name, cmp),
                }
                if data.base_id() == target.base_id() {
                    same_base += 1;
                }
            } else {
                if self.print_missing {
                    println!("Missing blocks: \"{}\"", name);
                }
                missing_count += 1;
            }
        }
        println!("=============\nMissing {} properties", missing_props);
        println!("Missing {} blocks", missing_count);
        println!("{} out of {} have the same base id", same_base, base_version.blocks().len() - missing_count);
        println!("{} perfect blocks", perfect_count);

        Ok(())
    }
}