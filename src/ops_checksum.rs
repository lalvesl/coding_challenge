use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn process_checksum(path: &Path) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let checksum = compute_checksum_from_reader(&mut reader)?;
    println!("{checksum}  {}", path.display());
    Ok(())
}

fn compute_checksum_from_reader<R: Read>(mut reader: R) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_checksum_hello() {
        let input = "hello";
        let reader = Cursor::new(input);
        let result = compute_checksum_from_reader(reader).unwrap();
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_checksum_empty() {
        let input = "";
        let reader = Cursor::new(input);
        let result = compute_checksum_from_reader(reader).unwrap();
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
