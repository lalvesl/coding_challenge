use anyhow::Result;

mod cli;
mod ops_checksum;
mod ops_parse;
mod runner;

fn main() -> Result<()> {
    runner::run()
}
