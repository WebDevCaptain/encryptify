# Encryptify Library

[![CI pipeline](https://github.com/WebDevCaptain/encryptify/actions/workflows/ci.yml/badge.svg?branch=workspace)](https://github.com/WebDevCaptain/encryptify/actions/workflows/ci.yml)

**Encryptify** is a library for encrypting and decrypting files and folders. It ensures the confidentiality of your data by using `AES encryption`.

> For folders, it compresses them into a ZIP archive before encrypting.

---

## Features

1. **File Encryption/Decryption**: Securely encrypt and decrypt individual files.

2. **Folder Encryption/Decryption**: Compress folders into ZIP archives before encrypting them.

3. **AES Encryption**: Supports AES-256 for strong security.

---

## Usage

This library crate exposes 4 functions for encryption and decryption and also provides a utility function to zip a folder.

1. **encrypt_file**: Encrypts a file using AES encryption. The new file will have a `.encrypted` extension.

2. **decrypt_file**: Decrypts an encrypted file using AES decryption. The new file will have a `.decrypted` extension.

3. **encrypt_folder**: Compresses a folder into a ZIP archive and encrypts it using AES encryption. The new file will have a `.zip.encrypted` extension.

4. **decrypt_folder**: Decrypts an encrypted ZIP archive and extracts it into a folder. The decrypted folder will not have anything appended to its name.

5. **zip_folder**: Compresses a folder into a ZIP archive. The new file will have a `.zip` extension.
