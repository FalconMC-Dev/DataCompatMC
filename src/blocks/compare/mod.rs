use anyhow::Context;
use clap::Args;
use crate::blocks::compare::kinds::PropertyComparison;
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
}

impl CompareCommand {
    pub fn compare<'raw>(&self) -> anyhow::Result<()> {
        let base_version: ModernBlockList = self.base.data().with_context(|| format!("Error deserializing {:?}", self.base.name()))?;
        let target_version: ModernBlockList = self.target.data().with_context(|| format!("Error deserializing {:?}", self.target.name()))?;

        eprintln!("Checking property compatibility...");
        for (property, base) in base_version.properties() {
            if let Some(target) = target_version.properties().get(property) {
                match PropertyComparison::compare(base, target) {
                    PropertyComparison::Equal => {}
                    cmp => eprintln!("\"{}\" has {:?}", property, cmp),
                }
            } else {
                eprintln!("Missing property: \"{}\"!", property);
            }
        }

        Ok(())
    }
}