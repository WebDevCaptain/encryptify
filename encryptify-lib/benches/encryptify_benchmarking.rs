use std::{
    fs::{self, File},
    io::Write,
};

use criterion::{Criterion, criterion_group, criterion_main};
use encryptify_lib::{encrypt_file, encrypt_folder};

fn setup_large_test_file(filepath: &str, size: usize) {
    let data = vec![b'a'; size];
    let mut file = File::create(filepath).expect("Failed to create test file");
    file.write_all(&data)
        .expect("Failed to write data to test file");
}

fn cleanup_files(file_paths: &[&str]) {
    for path in file_paths {
        if fs::metadata(path).is_ok() {
            fs::remove_file(path).expect("Failed to cleanup test file");
        }
    }
}

fn benchmark_encrypt_file(c: &mut Criterion) {
    let file_path = "large_test_file.txt";
    let key = [0u8; 32]; // AES-256 encryption key

    setup_large_test_file(file_path, 10 * 1024 * 1024); // 10 mb for now

    c.bench_function("encrypt_file_10_MB", |b| {
        b.iter(|| encrypt_file(file_path, &key))
    });

    cleanup_files(&[file_path, &format!("{}.encrypted", file_path)]);
}

fn benchmark_encrypt_folder(c: &mut Criterion) {
    let folder_path = "benchmarking_test_folder";
    let key = [0u8; 32]; // AES-256 encryption key

    fs::create_dir(folder_path).expect("Failed to create test folder");

    for i in 0..10 {
        setup_large_test_file(&format!("{}/file-{}.txt", folder_path, i), 1024 * 1024);
        // 1 MB file
    }

    c.bench_function("encrypt_folder_10_files_1_MB_each", |b| {
        b.iter(|| encrypt_folder(folder_path, &key))
    });

    cleanup_files(&[&format!("{}.zip.encrypted", folder_path)]);

    for i in 0..10 {
        fs::remove_file(format!("{}/file-{}.txt", folder_path, i))
            .expect("Failed to cleanup test file");
    }

    fs::remove_dir(folder_path).expect("Failed to cleanup test folder");
}

criterion_group!(benches, benchmark_encrypt_file, benchmark_encrypt_folder);
criterion_main!(benches);
