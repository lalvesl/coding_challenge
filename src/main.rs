use anyhow::Result;

mod cli;
mod ops;
mod runner;

fn main() -> Result<()> {
    runner::run()
}
