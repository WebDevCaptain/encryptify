use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use std::{
    fs::{self, File},
    io::{Read, Write},
};
use zip::{
    write::{ExtendedFileOptions, FileOptions},
    ZipWriter,
};

fn main() {
    let key = "password12345678password12345678".as_bytes();
    // let encrypted_file_path = "sample-file.txt.encrypted";

    // encrypt_file("sample-file.txt", &key);

    // decrypt_file(encrypted_file_path, &key);

    // encrypt_folder("vault", &key);

    decrypt_folder("vault.zip.encrypted", key);
}

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn encrypt_file(file_path: &str, key: &[u8]) {
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

fn decrypt_file(file_path: &str, key: &[u8]) {
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

fn zip_folder(folder_path: &str, output_path: &str) {
    let file = File::create(output_path).expect("Failed to create the output ZIP file");

    let mut zip = ZipWriter::new(file);

    let options: FileOptions<'_, ExtendedFileOptions> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in fs::read_dir(folder_path).expect("Failed to read files from the directory.") {
        let entry = entry.expect("Failed to get the entry");
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            zip.start_file(file_name, options.clone())
                .expect("Failed to add the file to zip");

            let mut f = File::open(path).expect("Failed to open the file");
            let mut buffer = Vec::new();

            f.read_to_end(&mut buffer).expect("Failed to read the file");

            zip.write_all(&buffer)
                .expect("Failed to write the file to zip...");
        }
    }

    zip.finish().expect("Failed to finalize the zip file");
}

fn encrypt_folder(folder_path: &str, key: &[u8]) {
    let zip_path = format!("{}.zip", folder_path);
    zip_folder(folder_path, &zip_path);
    encrypt_file(&zip_path, key);
    fs::remove_file(&zip_path).expect("Failed to remove the intermediate zip file :|");
}

fn decrypt_folder(file_path: &str, key: &[u8]) {
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
