use ::glob;
use ::zip;
use std::fs;

use zip_info::WriteZipInfo;
use zip_info::StatArgs;

/// Info Writer for multiple archive files:
pub struct MultiArchiveFlatWriter<'a> {
    path_names: &'a [String],
}

impl<'a> MultiArchiveFlatWriter<'a> {
    pub fn new(file_paths: &'a [String]) -> MultiArchiveFlatWriter<'a> {
        MultiArchiveFlatWriter { path_names: file_paths }
    }
}

impl<'a> WriteZipInfo for MultiArchiveFlatWriter<'a> {
    /// Given path names for multiple archives, concatenate their
    /// names, contents, and stats:
    fn write_zip_info(&mut self, exclude: &str, stat_args: &StatArgs) -> String {
        let mut output = Vec::new();

        for path_name in self.path_names {
            let archive_info = ZipInfoFlatWriter::new(path_name.as_str())
                .write_zip_info(exclude, stat_args);

            output.push(archive_info);
        }

        output.join("\n")
    }
}

pub struct ZipInfoFlatWriter<'a> {
    archive: zip::ZipArchive<fs::File>,
    path_name: &'a str,
}

impl<'a> ZipInfoFlatWriter<'a> {
    /// Open a zip archive for this writer:
    pub fn new(file_path: &str) -> ZipInfoFlatWriter {
        let file = fs::File::open(file_path).unwrap();

        let whole_archive = zip::ZipArchive::new(file).unwrap();

        ZipInfoFlatWriter { archive: whole_archive, path_name: file_path }
    }
}

impl<'a> WriteZipInfo for ZipInfoFlatWriter<'a> {
    /// Concatenate Zip file name with indented stats for an archive:
    fn write_zip_info(&mut self, exclude: &str, stat_args: &StatArgs) -> String {
        let mut info = format!("{}", self.path_name);

        let exclude_pattern = glob::Pattern::new(exclude).unwrap();

        for i in 0..self.archive.len() {
            let archive_item = self.archive.by_index(i).unwrap();

            if !exclude_pattern.matches(archive_item.name()) {
                info = format!("{}{}", info, info_for_archive_item(archive_item, stat_args));
            }
        }

        info
    }
}

fn info_for_archive_item(archive_item: zip::read::ZipFile, stat_args: &StatArgs) -> String {
    let mut info = String::new();
    let item_path = archive_item.name();
    info = format!("{}\n\t{}", info, item_path);

    if stat_args.flag_compression_type {
        let compression_type = archive_item.compression();
        info = format!("{}\n\t\tCompression type: {}", info, compression_type);
    }

    // Can't think of a way to avoid evaluating original_size or compressed_size without dependent typing...
    let original_size = archive_item.size();
    if stat_args.flag_original_size {
        info = format!("{}\n\t\tOriginal size: {}", info, original_size);
    }

    let compressed_size = archive_item.compressed_size();
    if stat_args.flag_compressed_size {
        info = format!("{}\n\t\tCompressed size: {}", info, compressed_size);
    }

    if stat_args.flag_compression_rate && original_size > 0 {
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
