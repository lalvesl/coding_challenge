use crate::cli::Cli;
use crate::ops::{process_checksum, process_parse};
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    for path in &cli.files {
        if cli.parse {
            process_parse(path)?;
        } else if cli.checksum {
            process_checksum(path)?;
        }
    }

    Ok(())
}
