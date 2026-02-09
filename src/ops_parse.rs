use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn process_parse(path: &Path) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);
    let output = parse_json_from_reader(reader)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))?;
    println!("{}", output);
    Ok(())
}

fn parse_json_from_reader<R: Read>(reader: R) -> Result<String> {
    let v: Value = serde_json::from_reader(reader)?;
    let s = serde_json::to_string_pretty(&v)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_json_valid() {
        let json = r#"{"foo":"bar"}"#;
        let reader = Cursor::new(json);
        let result = parse_json_from_reader(reader).unwrap();
        assert!(result.contains("\"foo\": \"bar\""));
    }

    #[test]
    fn test_parse_json_invalid() {
        let json = r#"{"foo":}"#;
        let reader = Cursor::new(json);
        let result = parse_json_from_reader(reader);
        assert!(result.is_err());
    }
}
