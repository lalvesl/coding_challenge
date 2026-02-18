use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_parse_matches_prettier() {
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/large_file.json");
    let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/test-data-gen/large_file_prettier.json");

    // Ensure files exist
    if !std::path::Path::new(&input_path).exists() || !std::path::Path::new(&expected_path).exists()
    {
        panic!("Test data missing. Please run 'cargo run --bin test-data-gen' first.");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_my_app"))
        .arg("--parse")
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let expected = fs::read_to_string(expected_path).expect("Failed to read expected file");

    assert_eq!(
        stdout, expected,
        "Output does not match prettier output! See target/test-data-gen/actual_output.json"
    );
}
