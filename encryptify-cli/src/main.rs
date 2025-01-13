use encryptify_lib::{decrypt_file, decrypt_folder, encrypt_file, encrypt_folder};

fn main() {
    let key = "password12345678password12345678".as_bytes();
    // let encrypted_file_path = "sample-file.txt.encrypted";

    // encrypt_file("sample-file.txt", &key);

    // decrypt_file(encrypted_file_path, &key);

    // encrypt_folder("vault", &key);

    decrypt_folder("vault.zip.encrypted", key);
}
