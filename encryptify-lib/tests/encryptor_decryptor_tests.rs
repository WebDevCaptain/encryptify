use std::{
    fs::{self, File},
    io::Write,
};

use encryptify_lib::{decrypt_file, encrypt_file};

// Utility functions
fn setup_test_file(file_path: &str, content: &[u8]) {
    let mut file = File::create(file_path).expect("Failed to create thetest file");
    file.write_all(content)
        .expect("Failed to write the test data.")
}

fn cleanup_files(file_paths: &[&str]) {
    for file_path in file_paths {
        fs::remove_file(file_path).expect("Failed to remove the test file");
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
