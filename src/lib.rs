//! # my_app
//!
//! `my_app` is a command-line tool for parsing JSON and computing SHA256 checksums.
//!
//! It supports:
//! - Streaming JSON parsing avoiding memory overhead.
//! - SHA256 checksum calculation for files.
//! - Input from both files and standard input.

pub mod arguments;
pub mod cli;
pub mod runner;
pub mod traits;
pub mod utils;
