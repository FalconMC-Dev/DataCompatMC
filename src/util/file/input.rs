use std::path::PathBuf;
use anyhow::Context;

use clap::{Arg, ArgMatches, Args, Command, Error, ErrorKind, FromArgMatches};
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
        // thread::sleep(Duration::from_millis(500));
        Ok(result)
    }
}

impl FromArgMatches for InputFile {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let file_name = PathBuf::from(matches.value_of_os("INPUT").ok_or(Error::raw(ErrorKind::EmptyValue, "No INPUT provided"))?);
        let contents = std::fs::read_to_string(&file_name)?;
        // thread::sleep(Duration::from_millis(500));
        Ok(Self {
            name: file_name,
            contents,
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let file_name = PathBuf::from(matches.value_of_os("INPUT").ok_or(Error::raw(ErrorKind::EmptyValue, "No INPUT provided"))?);
        let contents = std::fs::read_to_string(&file_name)?;
        // thread::sleep(Duration::from_millis(500));
        self.name = file_name;
        self.contents = contents;
        Ok(())
    }
}

impl Args for InputFile {
    fn augment_args(cmd: Command<'_>) -> Command<'_> {
        cmd.arg(Arg::new("INPUT")
            .help("Input file")
            .required(true)
            .takes_value(true)
            .allow_invalid_utf8(true))
    }

    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd.arg(Arg::new("INPUT")
            .help("Input file")
            .required(true)
            .takes_value(true)
            .allow_invalid_utf8(true))
    }
}