use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_parse_multiple_files() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen");
    let valid_json = target_dir.join("valid.json");

    // Create another file
    let other_json = target_dir.join("other.json");
    let mut file = fs::File::create(&other_json).unwrap();
    writeln!(file, "{{ \"a\": 1 }}").unwrap();

    let output = Command::new(bin_path)
        .arg("--parse")
        .arg(&valid_json)
        .arg(&other_json)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("{\n  \"foo\": \"bar\"\n}"));
    assert!(stdout.contains("{\n  \"a\": 1\n}"));

    fs::remove_file(other_json).unwrap();
}

#[test]
fn test_checksum_multiple_files() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen");
    let checksum_txt = target_dir.join("checksum.txt");

    // Create another file
    let other_txt = target_dir.join("other.txt");
    let mut file = fs::File::create(&other_txt).unwrap();
    writeln!(file, "other").unwrap();

    let output = Command::new(bin_path)
        .arg("--checksum")
        .arg(&checksum_txt)
        .arg(&other_txt)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains(&checksum_txt.display().to_string()));
    assert!(stdout.contains(&other_txt.display().to_string()));

    fs::remove_file(other_txt).unwrap();
}

#[test]
fn test_filter_directories() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen");
    let checksum_txt = target_dir.join("checksum.txt");

    // Pass a directory
    let output = Command::new(bin_path)
        .arg("--checksum")
        .arg(&checksum_txt)
        .arg(&target_dir) // This is a directory, should be skipped
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain checksum of the file
    assert!(stdout.contains(&checksum_txt.display().to_string()));

    let lines: Vec<&str> = stdout.trim().split('\n').collect();
    assert_eq!(lines.len(), 1, "Expected exactly one line of output");
}
