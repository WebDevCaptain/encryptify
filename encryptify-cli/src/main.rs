use clap::{Arg, Command};
use std::path::Path;

use encryptify_lib::{decrypt_file, decrypt_folder, encrypt_file, encrypt_folder};

fn main() {
    let matches =
        Command::new("Encryptify")
            .version("1.0.0")
            .author("WebDevCaptain")
            .about("Encrypt and Decrypt files & folders with Rust 🔥")
            .arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_parser(["encrypt", "decrypt"])
                    .required(true)
                    .help("Mode: encrypt or decrypt"),
            )
            .arg(
                Arg::new("path")
                    .short('p')
                    .long("path")
                    .required(true)
                    .help("Path to file or folder"),
            )
            .arg(
                Arg::new("key").short('k').long("key").required(true).help(
                    "Encryption key (32 characters long). Eg: helloworld123456helloworld123456",
                ),
            )
            .get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let path = matches.get_one::<String>("path").unwrap();
    let key = matches.get_one::<String>("key").unwrap();
    let key_bytes = key.as_bytes();

    if mode == "encrypt" {
        if Path::new(path).is_dir() {
            encrypt_folder(path, key_bytes);
        } else {
            encrypt_file(path, key_bytes);
        }
    } else if mode == "decrypt" {
        if path.ends_with(".zip.encrypted") {
            decrypt_folder(path, key_bytes);
        } else {
            decrypt_file(path, key_bytes);
        }
    }
}
