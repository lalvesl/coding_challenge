use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

        // Skip directories or non-files if any
        if !input_path.is_file() {
            continue;
        }

        // Get strict path string for commands
        let input_path_str = input_path.to_str().unwrap();

        // Run system sha256sum
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
