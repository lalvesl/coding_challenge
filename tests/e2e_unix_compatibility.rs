use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[test]
fn test_sha256sum_compatibility() {
    let test_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data-gen/compat_test_dir");
    // Iterate over existing files in compat_test_dir
    let entries = fs::read_dir(test_dir)
        .expect("Failed to read compat_test_dir. Did you run 'nix run .#prepare_tests'?");

    for entry in entries {
        let entry = entry.unwrap();
        let input_path = entry.path();

        if !input_path.is_file() {
            continue;
        }

        let input_path_str = input_path.to_str().unwrap();

        let sha256_output = Command::new("sha256sum")
            .arg(input_path_str)
            .output()
            .expect("Failed to execute sha256sum");

        if !sha256_output.status.success() {
            eprintln!(
                "sha256sum command failed or not found, skipping compatibility test for {:?}",
                input_path
            );
            continue;
        }

        let sha256_stdout = String::from_utf8(sha256_output.stdout).unwrap();

        // Run my_app
        let my_app_output = Command::new(env!("CARGO_BIN_EXE_my_app"))
            .arg("--checksum")
            .arg(input_path_str)
            .output()
            .expect("Failed to execute my_app");

        assert!(
            my_app_output.status.success(),
            "my_app failed for file {:?}",
            input_path
        );
        let my_app_stdout = String::from_utf8(my_app_output.stdout).unwrap();

        // Compare outputs
        assert_eq!(
            my_app_stdout,
            sha256_stdout,
            "Output mismatch for file {:?} (size {})",
            input_path,
            entry.metadata().unwrap().len()
        );
    }
}

#[test]
fn test_pipe_checksum_valid() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let input = "hello";

    let mut child = Command::new(bin_path)
        .arg("--checksum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    let mut sha256sum = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    {
        let stdin = sha256sum.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let sha256sum_output = sha256sum.wait_with_output().expect("Failed to read output");
    assert!(sha256sum_output.status.success());
    let sha256sum_stdout = String::from_utf8(sha256sum_output.stdout).unwrap();

    assert_eq!(stdout, sha256sum_stdout);
}
