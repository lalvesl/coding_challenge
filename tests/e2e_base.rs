use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[test]
fn test_parse_valid_json() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/valid.json");

    let output = Command::new(bin_path)
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // serde_json pretty prints with 2 spaces
    assert!(stdout.contains("{\n  \"foo\": \"bar\"\n}"));
}

#[test]
fn test_parse_invalid_json() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/invalid.json");

    let output = Command::new(bin_path)
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to parse JSON"));
}

#[test]
fn test_checksum_valid_file() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/checksum.txt");

    let output = Command::new(bin_path)
        .arg("--checksum")
        .arg(&input_path)
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8(output.stderr).unwrap()
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // sha256 of "hello"
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
    assert!(stdout.contains(&input_path.display().to_string()));
}

#[ignore]
#[test]
fn test_file_not_found() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/non_existent.txt");
    if std::path::Path::new(&input_path).exists() {
        fs::remove_file(&input_path).unwrap();
    }

    let output = Command::new(bin_path)
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success()); // Should be success now as it skips
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("Failed to open file"));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.is_empty());
}

#[test]
fn test_missing_mode() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/missing_mode.json");

    let output = Command::new(bin_path)
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    // Error message from clap
    let stderr = String::from_utf8(output.stderr).unwrap();
    // clap prints "Usage: my_app <COMMAND>" or similar error when subcommand is required

    assert!(
        stderr.contains("unrecognized subcommand") || stderr.contains("Failed to parse arguments")
    );
}

#[test]
fn test_parse_flag_valid_json() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/valid.json");

    let output = Command::new(bin_path)
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("{\n  \"foo\": \"bar\"\n}"));
}

#[test]
fn test_checksum_flag_valid_file() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/checksum.txt");

    let output = Command::new(bin_path)
        .arg("--checksum")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
}

#[test]
fn test_pipe_parse_valid() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let json = r#"{"foo":"bar"}"#;

    let mut child = Command::new(bin_path)
        .arg("--parse")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(json.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("{\n  \"foo\": \"bar\"\n}"));
}
