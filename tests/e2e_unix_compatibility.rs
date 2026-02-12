use rand::RngExt;
use std::fs;
use std::process::Command;

const TEST_DIR: &str = "tests/compat_test_dir";

#[test]
fn test_sha256sum_compatibility() {
    let mut rng = rand::rng();

    if std::path::Path::new(TEST_DIR).exists() {
        fs::remove_dir_all(TEST_DIR).unwrap();
    }
    fs::create_dir_all(TEST_DIR).unwrap();

    // Run 10 iterations with random content
    for i in 0..10 {
        // Generate random filename
        let file_name = format!("file_{}_{}.bin", i, rng.random::<u32>());
        let input_path = format!("{}/{}", TEST_DIR, file_name);

        // Generate random size (between 0 and 10kB)
        let size = rng.random_range(0..10240);
        let mut content = vec![0u8; size];
        rng.fill(&mut content[..]);

        fs::write(&input_path, &content).unwrap();

        let sha256_output = Command::new("sha256sum")
            .arg(&input_path)
            .output()
            .expect("Failed to execute sha256sum");

        if !sha256_output.status.success() {
            eprintln!("sha256sum command failed or not found, skipping compatibility test");
            fs::remove_dir_all(TEST_DIR).unwrap();
            return;
        }

        let sha256_stdout = String::from_utf8(sha256_output.stdout).unwrap();

        let my_app_output = Command::new(env!("CARGO_BIN_EXE_my_app"))
            .arg("--checksum")
            .arg(&input_path)
            .output()
            .expect("Failed to execute my_app");

        assert!(
            my_app_output.status.success(),
            "my_app failed for file {}",
            input_path
        );
        let my_app_stdout = String::from_utf8(my_app_output.stdout).unwrap();

        // Compare outputs
        assert_eq!(
            my_app_stdout, sha256_stdout,
            "Output mismatch for file {} (size {})",
            input_path, size
        );
    }

    // Cleanup
    fs::remove_dir_all(TEST_DIR).unwrap();
}
