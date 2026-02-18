use anyhow::Result;
use sha2::{Digest, Sha256};
use std::io::{self, Read, Write};

use crate::traits::CommandArg;
use crate::utils::process_inputs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ChecksumCommand {
    pub files: Vec<PathBuf>,
}

impl ChecksumCommand {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CommandArg for ChecksumCommand {
    fn name(&self) -> &'static str {
        "checksum"
    }

    fn build(&self) -> clap::Arg {
        clap::Arg::new(self.name())
            .long(self.name())
            .help("Print the checksum of the file contents")
            .num_args(0..)
            .value_parser(clap::value_parser!(PathBuf))
    }

    fn run(&self, matches: &clap::ArgMatches, writer: &mut dyn std::io::Write) -> Result<()> {
        if matches.contains_id(self.name()) {
            let files = matches
                .get_many::<PathBuf>(self.name())
                .map(|v| v.cloned().collect::<Vec<_>>())
                .unwrap_or_default();

            process_inputs(&files, writer, |mut reader, path_display, writer| {
                process_checksum_internal(&mut reader, path_display, writer)
            })?;
        }
        Ok(())
    }
}

pub fn process_checksum_internal<R: Read, W: Write>(
    mut reader: R,
    path_display: &str,
    mut writer: W,
) -> Result<()> {
    let mut buffer = HashWriter {
        hasher: Sha256::new(),
    };
    io::copy(&mut reader, &mut buffer)?;
    let result = buffer.hasher.finalize();
    let checksum = hex::encode(result);
    writeln!(writer, "{}  {}", &checksum, path_display)?;
    Ok(())
}

struct HashWriter {
    hasher: Sha256,
}

impl Write for HashWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_checksum_hello() {
        let input = "hello";
        let reader = Cursor::new(input);
        let path = "test_file.txt";
        let mut writer = Vec::new();
        process_checksum_internal(reader, path, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        let expected_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert_eq!(result, format!("{}  {}\n", expected_hash, path));
    }

    #[test]
    fn test_checksum_empty() {
        let input = "";
        let reader = Cursor::new(input);
        let path = "empty_file";
        let mut writer = Vec::new();
        process_checksum_internal(reader, path, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        let expected_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(result, format!("{}  {}\n", expected_hash, path));
    }
}
