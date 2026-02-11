use anyhow::Result;

mod cli;
mod json_formatter;
mod ops_checksum;
mod ops_parse;
mod runner;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    runner::run(std::env::args(), &mut stdout)
}
