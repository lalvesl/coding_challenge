use crate::cli::Cli;
use crate::ops_checksum::process_checksum;
use crate::ops_parse::process_parse;
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
