use clap::{ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "my_app")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to parse JSON or compute checksums", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["parse", "checksum"]),
))]
pub struct Cli {
    /// Pretty-print parsed JSON
    #[arg(long, group = "mode")]
    pub parse: bool,

    /// Print the checksum of the file contents
    #[arg(long, group = "mode")]
    pub checksum: bool,

    /// Input files
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mode() {
        let args = vec!["my_app", "--parse", "file.json"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.parse);
        assert!(!cli.checksum);
        assert_eq!(cli.files[0].to_str().unwrap(), "file.json");
    }

    #[test]
    fn test_checksum_mode() {
        let args = vec!["my_app", "--checksum", "file.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.checksum);
        assert!(!cli.parse);
        assert_eq!(cli.files[0].to_str().unwrap(), "file.txt");
    }

    #[test]
    fn test_multiple_files() {
        let args = vec!["my_app", "--parse", "file1.json", "file2.json"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.parse);
        assert_eq!(cli.files.len(), 2);
    }

    #[test]
    fn test_missing_mode_fails() {
        let args = vec!["my_app", "file.json"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_conflict_fails() {
        let args = vec!["my_app", "--parse", "--checksum", "file.json"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }
}
