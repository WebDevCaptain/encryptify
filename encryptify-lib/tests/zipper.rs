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

    // Intentionally failing the test
    assert!(false);

    // Cleanup: Remove the test folder and zip file
    fs::remove_dir_all(folder_path).unwrap();
    fs::remove_file(output_path).unwrap();
}
