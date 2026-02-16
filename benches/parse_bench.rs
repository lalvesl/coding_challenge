use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use my_app::commands::parse::{ParseCommand, process_parse_internal};
use my_app::traits::Runnable;
use std::fs;
use std::io::{Cursor, Sink};
use std::path::PathBuf;
use std::time::Duration;

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    // Ensure we have a long enough measurement time for file I/O
    group.measurement_time(Duration::from_secs(10));

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_dir = project_root.join("target/test-data-gen");
    let large_file_path = target_dir.join("large_file.json");

    // Check if file exists, if not, we might fail or warn.
    // Ideally we should run test-data-gen before this, but for now let's assume it exists
    // or panic with a helpful message.
    if !large_file_path.exists() {
        panic!(
            "Test data not found at {:?}. Please run 'cargo run -p test-data-gen' first.",
            large_file_path
        );
    }

    let content = fs::read(&large_file_path).expect("Failed to read test data");
    group.throughput(Throughput::Bytes(content.len() as u64));

    group.bench_function("memory_process_parse_internal", |b| {
        b.iter(|| {
            let reader = Cursor::new(&content);
            let mut writer = Sink::default(); // Discard output
            process_parse_internal(reader, &mut writer).unwrap();
        })
    });

    group.bench_function("pipe_stdin_parse", |b| {
        b.iter(|| {
            let reader = Cursor::new(&content);
            let mut writer = Sink::default();
            // Simulate stdin by passing a cursor directly to the internal processing function
            process_parse_internal(reader, &mut writer).unwrap();
        })
    });

    group.bench_function("file_process_parse", |b| {
        b.iter(|| {
            let mut writer = Sink::default();
            let cmd = ParseCommand {
                files: vec![large_file_path.clone()],
            };
            cmd.run(&mut writer).unwrap();
        })
    });

    group.finish();
}
criterion_group!(benches, bench_parse);
criterion_main!(benches);
