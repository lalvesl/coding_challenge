use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "my_app")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to parse JSON or compute checksums", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["parse", "checksum"]),
))]
struct Cli {
    /// Pretty-print parsed JSON
    #[arg(long, group = "mode")]
    parse: bool,

    /// Print the checksum of the file contents
    #[arg(long, group = "mode")]
    checksum: bool,

    /// Input files
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn process_parse(path: &Path) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    let v: Value = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))?;

    println!("{}", serde_json::to_string_pretty(&v)?);
    Ok(())
}

fn process_checksum(path: &Path) -> Result<()> {
    if path.is_dir() {
        eprintln!("{}: Is a directory", path.display());
        return Ok(());
    }

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
    let result = hex::encode(result);
    println!("{result}  {}", path.display());
    Ok(())
}

fn main() -> Result<()> {
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
