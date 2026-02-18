pub trait CommandArg {
    fn name(&self) -> &'static str;
    fn build(&self) -> clap::Arg;
    fn run(
        &self,
        matches: &clap::ArgMatches,
        writer: &mut dyn std::io::Write,
    ) -> anyhow::Result<()>;
}
