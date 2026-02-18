use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use my_app::commands::parse::process_parse_internal;
use std::fs;
use std::io::{Cursor, Sink};
use std::path::PathBuf;
use std::time::Duration;

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    group.measurement_time(Duration::from_secs(10));

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_dir = project_root.join("target/test-data-gen");
    let large_file_path = target_dir.join("large_file.json");

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
            let files = vec![large_file_path.clone()];
            my_app::utils::process_inputs(&files, &mut writer, |reader, _path_display, writer| {
                my_app::commands::parse::process_parse_internal(reader, writer)
            })
            .unwrap();
        })
    });

    group.finish();
}
criterion_group!(benches, bench_parse);
criterion_main!(benches);
