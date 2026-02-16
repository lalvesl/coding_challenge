use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use crate::traits::Runnable;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ParseCommand {
    /// Input file(s) (read from stdin if not provided)
    #[arg(name = "FILE")]
    pub files: Vec<PathBuf>,
}

impl Runnable for ParseCommand {
    fn run<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.files.is_empty() {
            let stdin = std::io::stdin();
            let reader = stdin.lock();
            process_parse_internal(reader, writer).context("Failed to parse JSON from stdin")?;
        } else {
            for path in &self.files {
                if !path.is_file() {
                    continue;
                }
                let file = File::open(path)
                    .with_context(|| format!("Failed to open file: {}", path.display()))?;
                let reader = BufReader::new(file);
                process_parse_internal(reader, &mut *writer)
                    .with_context(|| format!("Failed to parse JSON: {}", path.display()))?;
            }
        }
        Ok(())
    }
}

pub fn process_parse_internal<R: Read, W: Write>(reader: R, mut writer: W) -> Result<()> {
    let v: Value = serde_json::from_reader(reader)?;
    let s = serde_json::to_string_pretty(&v)?;
    write!(writer, "{}", s)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_json_valid() {
        let json = r#"{"foo":"bar"}"#;
        let reader = Cursor::new(json);
        let mut writer = Vec::new();
        process_parse_internal(reader, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        // serde_json::to_string_pretty defaults to 2 spaces indentation
        let expected = "{\n  \"foo\": \"bar\"\n}";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_json_invalid() {
        let json = r#"{"foo":}"#;
        let reader = Cursor::new(json);
        let mut writer = Vec::new();
        let result = process_parse_internal(reader, &mut writer);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_parse_file_not_found() {
        let cmd = ParseCommand {
            files: vec![PathBuf::from("non_existent_file.json")],
        };
        let mut writer = Vec::new();
        let result = cmd.run(&mut writer);
        // It skips non-existent files, so result is Ok and writer is empty
        assert!(result.is_ok());
        assert!(writer.is_empty());
    }
}
