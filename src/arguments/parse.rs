use anyhow::{Context, Result};

use std::io::{Read, Write};
use std::path::PathBuf;

use crate::traits::CommandArg;
use crate::utils::process_inputs;

#[derive(Debug, Default)]
pub struct ParseArgument {
    pub files: Vec<PathBuf>,
}

impl ParseArgument {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CommandArg for ParseArgument {
    fn name(&self) -> &'static str {
        "parse"
    }

    fn build(&self) -> clap::Arg {
        clap::Arg::new(self.name())
            .long(self.name())
            .help("Pretty-print parsed JSON")
            .num_args(0..)
            .value_parser(clap::value_parser!(PathBuf))
    }

    fn run(&self, matches: &clap::ArgMatches, writer: &mut dyn std::io::Write) -> Result<()> {
        if matches.contains_id(self.name()) {
            let files = matches
                .get_many::<PathBuf>(self.name())
                .map(|v| v.cloned().collect::<Vec<_>>())
                .unwrap_or_default();

            process_inputs(&files, writer, |reader, path_display, writer| {
                process_parse_internal(reader, writer)
                    .with_context(|| format!("Failed to parse JSON: {}", path_display))
            })?;
        }
        Ok(())
    }
}

pub fn process_parse_internal<R: Read, W: Write>(reader: R, writer: W) -> Result<()> {
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let mut serializer = serde_json::Serializer::pretty(writer);
    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
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
        let expected = "{\n  \"foo\": \"bar\"\n}";
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
