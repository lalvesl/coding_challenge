use anyhow::Result;
use clap::{self, Parser};

use crate::arguments::arguments;

#[derive(Parser, Debug)]
#[command(name = "my_app")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to parse JSON or compute checksums", long_about = None)]
pub struct Cli {}

impl Cli {
    pub fn run_from<I, T>(args: I, writer: &mut dyn std::io::Write) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let arguments = arguments();
        let mut app = clap::Command::new("my_app")
            .version("0.1.0")
            .about("A CLI tool to parse JSON or compute checksums");

        for arg in &arguments {
            app = app.arg(arg.build());
        }

        // Add completions command
        app = app.subcommand(
            clap::Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    clap::Arg::new("shell")
                        .value_parser(clap::value_parser!(clap_complete::Shell))
                        .required(true),
                ),
        );

        // Add man command
        app = app.subcommand(
            clap::Command::new("man").about("Generate man pages").arg(
                clap::Arg::new("out_dir")
                    .short('o')
                    .long("out")
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .default_value("."),
            ),
        );

        let matches = app.clone().try_get_matches_from(args)?;

        // Handle subcommands first (completions, man)
        match matches.subcommand() {
            Some(("completions", sub_matches)) => {
                let shell = sub_matches
                    .get_one::<clap_complete::Shell>("shell")
                    .unwrap();
                clap_complete::generate(*shell, &mut app, "my_app", writer);
                return Ok(());
            }
            Some(("man", sub_matches)) => {
                let out_dir = sub_matches
                    .get_one::<std::path::PathBuf>("out_dir")
                    .unwrap();
                std::fs::create_dir_all(out_dir)?;
                let mut man_file = std::fs::File::create(out_dir.join("my_app.1"))?;
                clap_mangen::Man::new(app).render(&mut man_file)?;
                writeln!(
                    writer,
                    "Man page generated at {:?}",
                    out_dir.join("my_app.1")
                )?;
                return Ok(());
            }
            _ => {}
        }

        // Handle arguments
        let mut found_command = false;
        for arg in &arguments {
            if matches.contains_id(arg.name()) {
                arg.run(&matches, writer)?;
                found_command = true;
            }
        }

        if !found_command && matches.subcommand_name().is_none() {
            anyhow::bail!("No command or flag specified");
        }

        Ok(())
    }
}
