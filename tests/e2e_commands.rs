use std::fs;
use std::process::Command;

#[test]
fn test_completions() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");

    let output = Command::new(bin_path)
        .arg("completions")
        .arg("bash")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("_my_app"));
}

#[test]
fn test_man_pages() {
    let bin_path = env!("CARGO_BIN_EXE_my_app");
    let out_dir = tempfile::tempdir().unwrap();
    let out_path = out_dir.path();

    let output = Command::new(bin_path)
        .arg("man")
        .arg("--out")
        .arg(out_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Man page generated at"));

    let man_file = out_path.join("my_app.1");
    assert!(man_file.exists());
    let content = fs::read_to_string(man_file).unwrap();
    assert!(content.contains(".TH my_app 1"));
}
