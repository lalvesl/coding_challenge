use anyhow::Result;
use my_app::runner;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    runner::run(std::env::args(), &mut stdout)
}
