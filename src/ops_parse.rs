use crate::json_formatter;
use anyhow::{Context, Result};
use serde::Deserializer;
use serde::de::Visitor;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub fn process_parse<W: Write>(path: &Path, writer: &mut W) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);
    process_parse_internal(reader, writer)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))?;
    Ok(())
}

fn process_parse_internal<R: std::io::Read, W: Write>(reader: R, mut writer: W) -> Result<()> {
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let formatter = StreamingFormatter {
        writer: &mut writer,
        indent_level: 0,
    };
    deserializer
        .deserialize_any(formatter)
        .context("Failed to deserialize")?;
    // Add final newline
    writeln!(writer)?;
    Ok(())
}

struct StreamingFormatter<'a, W: Write> {
    writer: &'a mut W,
    indent_level: usize,
}

impl<'a, 'de, W: Write> Visitor<'de> for StreamingFormatter<'a, W> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any valid JSON value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        write!(self.writer, "{}", v).map_err(serde::de::Error::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        write!(self.writer, "{}", v).map_err(serde::de::Error::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        write!(self.writer, "{}", v).map_err(serde::de::Error::custom)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        write!(self.writer, "{}", v).map_err(serde::de::Error::custom)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::to_writer(&mut *self.writer, v).map_err(serde::de::Error::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        write!(self.writer, "null").map_err(serde::de::Error::custom)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        write!(self.writer, "null").map_err(serde::de::Error::custom)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        write!(self.writer, "[").map_err(serde::de::Error::custom)?;

        writeln!(self.writer).map_err(serde::de::Error::custom)?;

        let mut first = true;
        while let Some(value) = seq.next_element::<Value>()? {
            if !first {
                writeln!(self.writer,).map_err(serde::de::Error::custom)?;
            }
            // Indent
            write!(self.writer, "{}", "  ".repeat(self.indent_level + 1))
                .map_err(serde::de::Error::custom)?;

            let s = json_formatter::format_recursive(&value, self.indent_level + 1);
            write!(self.writer, "{}", s).map_err(serde::de::Error::custom)?;

            first = false;
        }

        if !first {
            writeln!(self.writer).map_err(serde::de::Error::custom)?;
        }

        write!(self.writer, "{}]", "  ".repeat(self.indent_level))
            .map_err(serde::de::Error::custom)?;
        Ok(())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        write!(self.writer, "{{").map_err(serde::de::Error::custom)?;
        writeln!(self.writer).map_err(serde::de::Error::custom)?;

        let mut first = true;
        while let Some(key) = map.next_key::<String>()? {
            if !first {
                writeln!(self.writer).map_err(serde::de::Error::custom)?;
            }
            let value = map.next_value::<Value>()?;

            // Indent
            write!(self.writer, "{}", "  ".repeat(self.indent_level + 1))
                .map_err(serde::de::Error::custom)?;

            // Key
            serde_json::to_writer(&mut *self.writer, &key).map_err(serde::de::Error::custom)?;
            write!(self.writer, ": ").map_err(serde::de::Error::custom)?;

            // Value
            let s = json_formatter::format_recursive(&value, self.indent_level + 1);
            write!(self.writer, "{}", s).map_err(serde::de::Error::custom)?;

            first = false;
        }

        if !first {
            writeln!(self.writer).map_err(serde::de::Error::custom)?;
        }

        write!(self.writer, "{}}}", "  ".repeat(self.indent_level))
            .map_err(serde::de::Error::custom)?;
        Ok(())
    }
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

    #[test]
    fn test_process_parse_file_not_found() {
        let path = Path::new("non_existent_file.json");
        let mut writer = Vec::new();
        let result = process_parse(path, &mut writer);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to open file")
        );
    }
}
