use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let output_dir = Path::new("target/test-data-generator");

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
    }

    // e2e_base.rs files
    create_file_if_missing(output_dir.join("valid.json"), r#"{"foo":"bar"}"#.as_bytes());
    create_file_if_missing(output_dir.join("invalid.json"), r#"{"foo":}"#.as_bytes());
    create_file_if_missing(output_dir.join("checksum.txt"), b"hello");
    create_file_if_missing(output_dir.join("conflict.json"), b"{}");
    create_file_if_missing(output_dir.join("missing_mode.json"), b"{}");

    // e2e_unix_compatibility.rs files
    let compat_dir = output_dir.join("compat_test_dir");
    if !compat_dir.exists() {
        fs::create_dir_all(&compat_dir).expect("Failed to create compat_test_dir");
    }

    // Check if compat dir is empty, if so generate files.
    // If files exist, we assume they are correct (or acceptable) to avoid re-generating.
    // The user requirement says "not generate if already exists".
    // Since these are random files, checking specific filenames is hard unless we regenerate filenames.
    // But if we regenerate filenames we might as well regenerate content.
    // Let's check if the directory has any files.
    if fs::read_dir(&compat_dir).unwrap().next().is_none() {
        println!("Generating compatibility test files...");
        for i in 0..10 {
            let random_suffix: u32 = rng.random();
            let file_name = format!("file_{}_{}.bin", i, random_suffix);
            let file_path = compat_dir.join(file_name);

            // Size between 1KB and 1MB
            let size = rng.random_range(1024..(1024 * 1024));
            let mut content = vec![0u8; size];
            rng.fill(&mut content[..]);

            create_file_if_missing(file_path, &content);
        }
    } else {
        println!("Compatibility test files already exist, skipping generation.");
    }

    println!("Test data generation complete.");
}

fn create_file_if_missing(path: PathBuf, content: &[u8]) {
    if path.exists() {
        println!("File {:?} already exists, skipping.", path);
        return;
    }
    println!("Creating file {:?}...", path);
    fs::write(&path, content).expect("Failed to write file");
}
