use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use crate::traits::Runnable;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ChecksumCommand {
    /// Input file(s) (read from stdin if not provided)
    #[arg(name = "FILE")]
    pub files: Vec<PathBuf>,
}

impl Runnable for ChecksumCommand {
    fn run<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.files.is_empty() {
            let stdin = std::io::stdin();
            let mut reader = stdin.lock();
            // Use "-" as filename for stdin, common convention
            process_checksum_internal(&mut reader, "-", writer)?;
        } else {
            for path in &self.files {
                if path.is_file() {
                    let file = File::open(path)
                        .with_context(|| format!("Failed to open file: {}", path.display()))?;
                    let mut reader = BufReader::new(file);
                    process_checksum_internal(
                        &mut reader,
                        &path.display().to_string(),
                        &mut *writer,
                    )?;
                } else {
                    eprintln!("{}: Is a directory", path.display().to_string());
                }
            }
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

    #[test]
    fn test_process_checksum_file_not_found() {
        let cmd = ChecksumCommand {
            files: vec![PathBuf::from("non_existent_file.txt")],
        };
        let mut writer = Vec::new();
        let result = cmd.run(&mut writer);
        // It skips non-existent files, so result is Ok and writer is empty
        assert!(result.is_ok());
        assert!(writer.is_empty());
    }
}
