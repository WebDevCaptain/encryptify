use std::{
    fs::{self, File},
    io::{Read, Write},
};

use zip::{
    write::{ExtendedFileOptions, FileOptions},
    ZipWriter,
};

pub fn zip_folder(folder_path: &str, output_path: &str) {
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
