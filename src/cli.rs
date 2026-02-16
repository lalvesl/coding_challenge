use crate::ops_checksum::ChecksumCommand;
use crate::ops_parse::ParseCommand;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "my_app")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to parse JSON or compute checksums", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Parse the file as JSON
    #[arg(long, group = "mode")]
    pub parse: bool,

    /// Calculate the checksum of the file
    #[arg(long, group = "mode")]
    pub checksum: bool,

    /// Input file (optional, used with flags)
    #[arg(name = "FILE", global = true)]
    pub file: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Pretty-print parsed JSON
    Parse(ParseCommand),

    /// Print the checksum of the file contents
    Checksum(ChecksumCommand),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mode() {
        let args = vec!["my_app", "parse", "file.json"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Some(Commands::Parse(cmd)) => {
                assert_eq!(cmd.file.unwrap().to_str().unwrap(), "file.json")
            }
            _ => panic!("Expected Parse command"),
        }
    }

    #[test]
    fn test_checksum_mode() {
        let args = vec!["my_app", "checksum", "file.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command {
            Some(Commands::Checksum(cmd)) => {
                assert_eq!(cmd.file.unwrap().to_str().unwrap(), "file.txt")
            }
            _ => panic!("Expected Checksum command"),
        }
    }

    #[test]
    fn test_parse_flag() {
        let args = vec!["my_app", "--parse", "file.json"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.parse);
        assert!(!cli.checksum);
        assert_eq!(cli.file.unwrap().to_str().unwrap(), "file.json");
    }

    #[test]
    fn test_checksum_flag() {
        let args = vec!["my_app", "--checksum", "file.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(!cli.parse);
        assert!(cli.checksum);
        assert_eq!(cli.file.unwrap().to_str().unwrap(), "file.txt");
    }

    #[test]
    fn test_no_args_parses_success() {
        let args = vec!["my_app"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.command.is_none());
        assert!(!cli.parse);
        assert!(!cli.checksum);
        assert!(cli.file.is_none());
    }
}
