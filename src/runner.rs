use crate::cli::Cli;
use crate::ops_checksum::process_checksum;
use crate::ops_parse::process_parse;
use anyhow::{Context, Result};
use clap::Parser;
use std::ffi::OsString;
use std::io::Write;

pub fn run<I, T, W>(args: I, writer: &mut W) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    W: Write,
{
    let cli = Cli::try_parse_from(args).context("Failed to parse arguments")?;

    for path in &cli.files {
        if cli.parse {
            process_parse(path, writer)?;
        } else if cli.checksum {
            process_checksum(path, writer)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_run_inner_parse() {
        let path = "test_run_inner_parse.json";
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "{{ \"foo\":\"bar\" }}").unwrap();

        let args = vec!["my_app", "--parse", path];
        let mut writer = Vec::new();
        run(args, &mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        // Streaming formatter expands top-level
        assert!(output.contains("\"foo\": \"bar\""));

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_run_inner_checksum() {
        let path = "test_run_inner_checksum.txt";
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "hello").unwrap();

        let args = vec!["my_app", "--checksum", path];
        let mut writer = Vec::new();
        run(args, &mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        assert!(
            output.contains("5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03")
        );

        std::fs::remove_file(path).unwrap();
    }
}
