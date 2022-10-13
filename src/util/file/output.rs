use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct OutputFile {
    /// Output file
    output: PathBuf,
}

impl OutputFile {
    pub fn writer(&self) -> anyhow::Result<Option<impl Write>> {
        if self.output.exists() {
            print!("{:?} already exists, do you want to overwrite it? [y/N] ", self.output);
            std::io::stdout().flush()?;
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer)?;
            if answer.trim().to_lowercase() != "y" {
                return Ok(None);
            }
        }
        Ok(Some(OpenOptions::new().create(true).write(true).truncate(true).open(&self.output)?))
    }
}

impl FromStr for OutputFile {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            output: PathBuf::from(s),
        })
    }
}
