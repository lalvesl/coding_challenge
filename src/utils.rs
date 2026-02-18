use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

pub fn process_inputs<W, F>(files: &[PathBuf], writer: &mut W, f: F) -> anyhow::Result<()>
where
    W: Write + ?Sized,
    F: Fn(Box<dyn Read>, &str, &mut W) -> Result<()>,
{
    if files.is_empty() {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        f(Box::new(reader), "-", writer)?;
    } else {
        for path in files {
            if path.is_file() {
                let file = File::open(path)
                    .with_context(|| format!("Failed to open file: {}", path.display()))?;
                let reader = BufReader::new(file);
                f(Box::new(reader), &path.display().to_string(), writer)?;
            } else {
                eprintln!("{}: Is a directory", path.display());
            }
        }
    }
    Ok(())
}
