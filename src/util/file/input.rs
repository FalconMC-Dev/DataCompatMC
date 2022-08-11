use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Context;

use serde::Deserialize;

#[derive(Debug)]
pub struct InputFile {
    name: PathBuf,
    contents: String,
}

impl InputFile {
    pub fn name(&self) -> &PathBuf {
        &self.name
    }

    pub fn data<'raw, T: Deserialize<'raw>>(&'raw self) -> anyhow::Result<T> {
        let result = serde_json::from_str::<T>(&self.contents).with_context(|| "Could not deserialize to the requested format")?;
        Ok(result)
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
