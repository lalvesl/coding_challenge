/// A trait for defining command-line arguments and their handlers.
pub trait CommandArg {
    /// Returns the name of the argument (used as the long flag).
    fn name(&self) -> &'static str;

    /// Builds the `clap::Arg` definition.
    fn build(&self) -> clap::Arg;

    /// Executes the logic associated with the argument if present.
    fn run(
        &self,
        matches: &clap::ArgMatches,
        writer: &mut dyn std::io::Write,
    ) -> anyhow::Result<()>;
}
