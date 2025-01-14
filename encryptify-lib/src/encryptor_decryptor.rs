use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use std::{
    fs::{self, File},
    io::Write,
};

use crate::zipper::zip_folder;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt_file(file_path: &str, key: &[u8]) {
    let file_content = fs::read(file_path).expect("Failed to read the file");

    let iv = [0u8; 16]; // Initialization vector

    let cipher = Aes256Cbc::new_from_slices(key, &iv).expect("Failed to create AES cipher");

    let cipher_text = cipher.encrypt_vec(&file_content);

    let mut output_file =
        File::create(format!("{}.encrypted", file_path)).expect("Failed to create the output file");

    output_file
        .write_all(&cipher_text)
        .expect("Failed to write encrypted data to the output file");
}

pub fn decrypt_file(file_path: &str, key: &[u8]) {
    let encrypted_content = fs::read(file_path).expect("Failed to read the encrypted file");

    let iv = [0u8; 16];

    let cipher = Aes256Cbc::new_from_slices(key, &iv).expect("Failed to create AES cipher");

    let decrypted_content = cipher
        .decrypt_vec(&encrypted_content)
        .expect("Failed to decrypt data");

    let mut output_file =
        File::create(format!("{}.decrypted", file_path)).expect("Failed to create the output file");

    output_file
        .write_all(&decrypted_content)
        .expect("Failed to write decrypted data to the output file");
}

pub fn encrypt_folder(folder_path: &str, key: &[u8]) {
    let zip_path = format!("{}.zip", folder_path);
    zip_folder(folder_path, &zip_path);
    encrypt_file(&zip_path, key);
    fs::remove_file(&zip_path).expect("Failed to remove the intermediate zip file :|");
}

pub fn decrypt_folder(file_path: &str, key: &[u8]) {
    decrypt_file(file_path, key);

    let zip_path = format!("{}.decrypted", file_path); // As the decrypt file function appends .decrypted to the file name

    let zip_file = File::open(&zip_path).expect("Failed to open the decrypted zip file");

    let mut archive = zip::ZipArchive::new(zip_file).expect("Failed to read the Zip archive");

    let output_folder = file_path.trim_end_matches(".zip.encrypted");

    fs::create_dir(output_folder).expect("Failed to create the output folder");

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .expect("Failed to get the file from the archive");

        let output_file_path = format!("{}/{}", output_folder, file.name()); // TODO: Fix security flaw here, needs sanitization

        let mut output_file =
            File::create(output_file_path).expect("Failed to create the output file");

        std::io::copy(&mut file, &mut output_file)
            .expect("Failed to copy contents to extracted file.");
    }

    fs::remove_file(&zip_path).expect("Failed to delete the intermediate 'decrypted' zip file");
}
