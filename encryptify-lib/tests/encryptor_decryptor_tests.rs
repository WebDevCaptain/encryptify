use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use encryptify_lib::{decrypt_file, decrypt_folder, encrypt_file, encrypt_folder};

// Utility functions
fn setup_test_file(file_path: &str, content: &[u8]) {
    let mut file = File::create(file_path).expect("Failed to create thetest file");
    file.write_all(content)
        .expect("Failed to write the test data.")
}

fn cleanup_files(file_paths: &[&str]) {
    for file_path in file_paths {
        // Check if folder
        if Path::new(file_path).is_dir() {
            fs::remove_dir_all(file_path).expect("Failed to remove the test folder");
        } else {
            // Delete the file

            fs::remove_file(file_path).expect("Failed to remove the test file");
        }
    }
}

#[test]
fn test_encrypt_and_decrypt_file() {
    let filepath = "test_file.txt";
    let key = [0u8; 32]; // 32 bytes key for AES-256
    let content = b"Encryptify is awesome! :) ";
    let encrypted_filepath = format!("{}.encrypted", filepath);
    let decrypted_filepath = format!("{}.encrypted.decrypted", filepath); // Decryption done after encryption, that's why the name

    setup_test_file(filepath, content);

    // Test encryption
    encrypt_file(filepath, &key);
    assert!(fs::metadata(&encrypted_filepath).is_ok());

    // Test decryption
    decrypt_file(&encrypted_filepath, &key);
    assert!(fs::metadata(&decrypted_filepath).is_ok());

    // Check the content of the decrypted file
    let decrypted_content =
        fs::read(&decrypted_filepath).expect("Failed to read the decrypted file");
    assert_eq!(decrypted_content, content);

    cleanup_files(&[filepath, &encrypted_filepath, &decrypted_filepath]);
}

#[test]
fn test_encrypt_decrypt_folder() {
    let folder_path = "test_folder";
    let key = [0u8; 32]; // 32 bytes key
    let file1_path = format!("{}/file1.txt", folder_path);
    let file2_path = format!("{}/file2.txt", folder_path);
    let content1 = b"Encryptify CLI is awesome! :) ";
    let content2 = b"Encryptify Library is awesome! :) ";

    // Check if folder already exists and remove it if it does
    if fs::metadata(folder_path).is_ok() {
        fs::remove_dir_all(folder_path).expect("Failed to remove the test folder");
    }

    // Create the folder and setup the test files
    fs::create_dir(folder_path).expect("Failed to create the test folder");
    setup_test_file(&file1_path, content1);
    setup_test_file(&file2_path, content2);

    // Test folder encryption
    encrypt_folder(folder_path, &key);
    assert!(fs::metadata(format!("{}.zip.encrypted", folder_path)).is_ok());

    // Delete the original folder to ensure that decrypt_folder creates a new one
    fs::remove_dir_all(folder_path).expect("Failed to remove the test folder");

    decrypt_folder(&format!("{}.zip.encrypted", folder_path), &key);

    let output_folder = folder_path;
    assert!(fs::metadata(output_folder).is_ok());

    let file_content1 = fs::read(&file1_path).expect("Failed to read the file1");
    assert_eq!(file_content1, content1);

    let file_content2 = fs::read(&file2_path).expect("Failed to read the file2");
    assert_eq!(file_content2, content2);

    // assert!(false);

    cleanup_files(&[
        &file1_path,
        &file2_path,
        output_folder,
        &format!("{}.zip.encrypted", folder_path),
    ]);
}
