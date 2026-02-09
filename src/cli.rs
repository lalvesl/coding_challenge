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
