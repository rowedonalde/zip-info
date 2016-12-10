extern crate zip;

use std::fs;
use zip_info::WriteZipInfo;

pub struct ZipInfoFlatWriter {
    archive: zip::ZipArchive<fs::File>,
}

impl ZipInfoFlatWriter {
    /// Open a zip archive for this writer:
    pub fn new(file_path: String) -> ZipInfoFlatWriter {
        let file = fs::File::open(&file_path).unwrap();

        let whole_archive = zip::ZipArchive::new(file).unwrap();

        ZipInfoFlatWriter { archive: whole_archive }
    }
}

impl WriteZipInfo for ZipInfoFlatWriter {
    fn write_zip_info(&mut self) -> String {
        let mut info = String::new();

        for i in 0..self.archive.len() {
            let archive_item = self.archive.by_index(i).unwrap();
            info = format!("{}{}", info, info_for_archive_item(archive_item));
        }

        info
    }
}

fn info_for_archive_item(archive_item: zip::read::ZipFile) -> String {
    let mut info = String::new();
    let item_path = archive_item.name();
    info = format!("{}\n\t{}", info, item_path);

    let compression_type = archive_item.compression();
    info = format!("{}\n\t\tCompression type: {}", info, compression_type);

    let original_size = archive_item.size();
    info = format!("{}\n\t\tOriginal size: {}", info, original_size);

    let compressed_size = archive_item.compressed_size();
    info = format!("{}\n\t\tCompressed size: {}", info, compressed_size);

    if original_size > 0 {
        let compression_rate =
            (original_size as f64 - compressed_size as f64)
            / original_size as f64;
        let compression_perc = format!("{:.*}%", 2, compression_rate * 100.0);
        info = format!("{}\n\t\tCompression_rate: {}", info, compression_perc);
    }

    let comment = archive_item.comment();
    if comment.len() > 0 {
        info = format!("{}\n\t\tComment: {}", info, comment);
    }

    info
}
