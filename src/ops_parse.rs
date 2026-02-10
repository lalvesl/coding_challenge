use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

pub fn process_parse(path: &Path) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut stdout = io::stdout();
    process_parse_internal(reader, &mut stdout)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))?;
    Ok(())
}

fn process_parse_internal<R: Read, W: Write>(reader: R, mut writer: W) -> Result<()> {
    let v: Value = serde_json::from_reader(reader)?;
    let s = serde_json::to_string_pretty(&v)?;
    writeln!(writer, "{}", s)?;
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
        let expected = "{\n  \"foo\": \"bar\"\n}\n";
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
}
