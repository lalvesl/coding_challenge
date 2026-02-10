use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

pub fn process_checksum<W: Write>(path: &Path, writer: &mut W) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    process_checksum_internal(&mut reader, &path.display().to_string(), writer)?;
    Ok(())
}

fn process_checksum_internal<R: Read, W: Write>(
    reader: R,
    path_display: &str,
    mut writer: W,
) -> Result<()> {
    let checksum = compute_checksum_from_reader(reader)?;
    writeln!(writer, "{}  {}", checksum, path_display)?;
    Ok(())
}

fn compute_checksum_from_reader<R: Read>(mut reader: R) -> Result<String> {
    let mut writer = HashWriter {
        hasher: Sha256::new(),
    };
    io::copy(&mut reader, &mut writer)?;
    let result = writer.hasher.finalize();
    Ok(hex::encode(result))
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
        let path = Path::new("non_existent_file.txt");
        let mut writer = Vec::new();
        let result = process_checksum(path, &mut writer);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to open file")
        );
    }
}
