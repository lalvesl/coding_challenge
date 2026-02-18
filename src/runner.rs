use crate::cli::Cli;
use anyhow::Result;
use std::ffi::OsString;
use std::io::Write;

/// Runs the application with the given arguments.
///
/// This is a convenience wrapper around `Cli::run_from`.
///
/// # Arguments
///
/// * `args` - Command line arguments.
/// * `writer` - Output writer.
pub fn run<I, T, W>(args: I, writer: &mut W) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    W: Write,
{
    match Cli::run_from(args, writer) {
        Ok(_) => Ok(()),
        Err(e) => {
            // clap errors return Error which can be printed
            match e.downcast_ref::<clap::Error>() {
                Some(clap_err) => {
                    write!(writer, "{}", clap_err.render())?;
                    if clap_err.use_stderr() {
                        return Err(anyhow::Error::msg("Failed to parse arguments"));
                    }
                    Ok(())
                }
                None => Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_run_inner_parse() {
        let path = "test_run_inner_parse.json";
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "{{ \"foo\":\"bar\" }}").unwrap();

        let args = vec!["my_app", "--parse", path];
        let mut writer = Vec::new();
        run(args, &mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        assert!(output.contains("\"foo\": \"bar\""));

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_run_inner_checksum() {
        let path = "test_run_inner_checksum.txt";
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "hello").unwrap();

        let args = vec!["my_app", "--checksum", path];
        let mut writer = Vec::new();
        run(args, &mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        assert!(
            output.contains("5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03")
        );

        std::fs::remove_file(path).unwrap();
    }
}
