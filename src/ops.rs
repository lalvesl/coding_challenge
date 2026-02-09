use anyhow::{Context, Result};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn process_parse(path: &Path) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    let v: Value = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))?;

    println!("{}", serde_json::to_string_pretty(&v)?);
    Ok(())
}

pub fn process_checksum(path: &Path) -> Result<()> {
    let mut file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let result = hasher.finalize();
    println!("{}", hex::encode(result));
    Ok(())
}
