use anyhow::Result;
use std::io::Write;

/// A trait for commands that can be run with a writer for output.
pub trait Runnable {
    fn run<W: Write>(&self, writer: &mut W) -> Result<()>;
}
