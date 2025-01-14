use std::fs::{self};
use std::path::Path;

use encryptify_lib::zip_folder;

#[test]
fn test_zip_folder_creates_zip_archive() {
    // Arrange: Setup a test folder and file
    let folder_path = "test-folder";
    let output_path = "test-folder.zip";
    fs::create_dir_all(folder_path).unwrap();
    fs::write(
        format!("{}/file.txt", folder_path),
        b"Hello, this is encryptify tool",
    )
    .unwrap();

    // Act: Zip the folder
    zip_folder(folder_path, output_path);

    // Assert: Check if the output zip file exists
    assert!(Path::new(output_path).exists());

    // Cleanup: Remove the test folder and zip file
    fs::remove_dir_all(folder_path).unwrap();
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_zip_folder_multiple_files() {
    // Arrange: Set up a test folder with multiple files
    let folder_path = "multi_file_test_folder";
    let output_path = "multi_file_test_folder.zip";
    fs::create_dir_all(folder_path).unwrap();
    std::fs::write(format!("{}/file1.txt", folder_path), b"File 1").unwrap();
    std::fs::write(format!("{}/file2.txt", folder_path), b"File 2").unwrap();

    // Act: Zip the folder
    zip_folder(folder_path, output_path);

    // Assert: Check if the output zip file exists
    assert!(Path::new(output_path).exists());

    // Cleanup
    fs::remove_file(output_path).unwrap();
    fs::remove_dir_all(folder_path).unwrap();
}

#[test]
fn test_zip_folder_handles_empty_folder() {
    let folder_path = "empty_test_folder";
    let output_path = "empty_test_folder.zip";
    fs::create_dir_all(folder_path).unwrap();

    zip_folder(folder_path, output_path);

    assert!(Path::new(output_path).exists());

    fs::remove_file(output_path).unwrap();
    fs::remove_dir_all(folder_path).unwrap();
}
