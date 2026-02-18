use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let output_dir = PathBuf::from("./target/test-data-gen");

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    // e2e_base.rs files
    create_file_if_missing(
        &output_dir.join("valid.json"),
        r#"{"foo":"bar"}"#.as_bytes(),
    );
    create_file_if_missing(&output_dir.join("invalid.json"), r#"{"foo":}"#.as_bytes());
    create_file_if_missing(&output_dir.join("checksum.txt"), b"hello");
    create_file_if_missing(&output_dir.join("conflict.json"), b"{}");
    create_file_if_missing(&output_dir.join("missing_mode.json"), b"{}");

    // e2e_unix_compatibility.rs files
    let compat_dir = output_dir.join("compat_test_dir");
    if !compat_dir.exists() {
        fs::create_dir_all(&compat_dir).expect("Failed to create compat_test_dir");
    }

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

            create_file_if_missing(&file_path, &content);
        }
    } else {
        println!("Compatibility test files already exist, skipping generation.");
    }

    // Generate large JSON file for prettier compatibility test
    let large_json_path = output_dir.join("large_file.json");
    if !large_json_path.exists() {
        println!("Generating large JSON file...");
        let mut large_json = String::from("[");
        for i in 0..100000 {
            large_json.push_str(&format!(
                "{{\"id\":{},\"name\":\"item_{}\",\"value\":{}}},",
                i,
                i,
                i * 2
            ));
        }

        large_json.truncate(large_json.len() - 1);
        large_json.push_str("]");
        create_file_if_missing(&large_json_path, large_json.as_bytes());

        let prettier_output_path = output_dir.join("large_file_prettier.json");
        fs::write(&prettier_output_path, &large_json).expect("Failed to write prettier input file");

        println!(
            "Running Prettier on {:?}... {}",
            prettier_output_path,
            std::path::Path::new(".").canonicalize().unwrap().display()
        );
        let status = std::process::Command::new("prettier")
            .arg("--config")
            .arg("test-data-gen/.prettierrc")
            .arg("--ignore-path")
            .arg("--write")
            .arg(&prettier_output_path)
            .status()
            .expect("Failed to run prettier");

        if !status.success() {
            eprintln!("Prettier failed with status: {}", status);
        }
    } else {
        println!("Large JSON file already exists, skipping.");
    }

    println!("Test data generation complete.");
}

fn create_file_if_missing(path: &PathBuf, content: &[u8]) {
    if path.exists() {
        println!("File {:?} already exists, skipping.", path);
        return;
    }
    println!("Creating file {:?}...", path);
    fs::write(path, content).expect("Failed to write file");
}
