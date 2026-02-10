use std::fs;
use std::process::Command;

#[test]
fn test_parse_valid_json() {
    let input_path = "tests/valid.json";
    let input_content = r#"{"foo":"bar"}"#;
    fs::write(input_path, input_content).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // serde_json pretty prints with 2 spaces
    assert!(stdout.contains(
        r#"{
  "foo": "bar"
}"#
    ));

    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_parse_invalid_json() {
    let input_path = "tests/invalid.json";
    let input_content = r#"{"foo":}"#;
    fs::write(input_path, input_content).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to parse JSON"));

    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_checksum_valid_file() {
    let input_path = "tests/checksum.txt";
    let input_content = "hello";
    fs::write(input_path, input_content).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg("--checksum")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // sha256 of "hello"
    assert!(stdout.contains("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
    assert!(stdout.contains(input_path));

    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_file_not_found() {
    let input_path = "tests/non_existent.txt";
    // Ensure file doesn't exist
    if std::path::Path::new(input_path).exists() {
        fs::remove_file(input_path).unwrap();
    }

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to open file"));
}

#[test]
fn test_arg_conflict() {
    let input_path = "tests/conflict.json";
    fs::write(input_path, "{}").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg("--parse")
        .arg("--checksum")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    // Clap error message for conflict
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("the argument '--parse' cannot be used with '--checksum'"));

    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_missing_mode() {
    let input_path = "tests/missing_mode.json";
    fs::write(input_path, "{}").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    // Clap error message for required group
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("the following required arguments were not provided"));

    fs::remove_file(input_path).unwrap();
}
