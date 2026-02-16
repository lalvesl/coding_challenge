use crate::cli::{Cli, Commands};
use crate::commands::checksum::ChecksumCommand;
use crate::commands::parse::ParseCommand;
use crate::traits::Runnable;
use anyhow::Result;
use clap::Parser;
use std::ffi::OsString;
use std::io::Write;

pub fn run<I, T, W>(args: I, writer: &mut W) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    W: Write,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(c) => c,
        Err(e) => {
            write!(writer, "{}", e.render())?;
            if e.use_stderr() {
                return Err(anyhow::Error::msg("Failed to parse arguments"));
            }
            return Ok(());
        }
    };

    match cli.command {
        Some(Commands::Parse(cmd)) => cmd.run(writer)?,
        Some(Commands::Checksum(cmd)) => cmd.run(writer)?,
        None => {
            if cli.parse {
                let cmd = ParseCommand { file: cli.file };
                cmd.run(writer)?;
            } else if cli.checksum {
                let cmd = ChecksumCommand { file: cli.file };
                cmd.run(writer)?;
            } else {
                // If no subcommand and no flag, print help or error
                // For now, let's print help using clap's mechanism or return an error
                // Since we don't have easy access to clap's help here without keeping the parser instance,
                // we'll return an error.
                anyhow::bail!("No command or flag specified");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_run_inner_parse() {
        let path = "test_run_inner_parse.json";
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "{{ \"foo\":\"bar\" }}").unwrap();

        let args = vec!["my_app", "parse", path];
        let mut writer = Vec::new();
        run(args, &mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        assert!(output.contains("\"foo\": \"bar\""));

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_run_inner_checksum() {
        let path = "test_run_inner_checksum.txt";
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "hello").unwrap();

        let args = vec!["my_app", "checksum", path];
        let mut writer = Vec::new();
        run(args, &mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        assert!(
            output.contains("5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03")
        );

        std::fs::remove_file(path).unwrap();
    }
}
