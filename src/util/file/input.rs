use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Context;

use serde::Deserialize;

/// This is the representation of an
/// input file to be parsed by clap. Upon
/// parsing, this will open and read the specified file if posssible.
#[derive(Debug)]
pub struct InputFile {
    name: PathBuf,
    contents: String,
}

impl InputFile {
    /// Returns the name of the file
    pub fn name(&self) -> &PathBuf {
        &self.name
    }

    /// Convenience function to automatically deserialize
    pub fn deserialized<'raw, T: Deserialize<'raw>>(&'raw self) -> anyhow::Result<T> {
        let result = serde_json::from_str::<T>(&self.contents).with_context(|| "Could not deserialize to the requested format")?;
        Ok(result)
    }

    /// Returns raw content of the file
    pub fn data(&self) -> &str {
        &self.contents
    }
}

impl FromStr for InputFile {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = PathBuf::from(s);
        let contents = std::fs::read_to_string(&name)?;
        Ok(Self {
            name,
            contents,
        })
    }
}
