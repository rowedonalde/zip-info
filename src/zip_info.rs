extern crate zip;

use std::fs;

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

fn info_for_archive(mut whole_archive: zip::ZipArchive<fs::File>) -> String {
    let mut info = String::new();

    for i in 0..whole_archive.len() {
        let archive_item = whole_archive.by_index(i).unwrap();
        info = format!("{}{}", info, info_for_archive_item(archive_item));
    }

    info
}

fn info_for_path(path: String) -> String {
    let mut info = String::new();
    info = format!("{}", path);

    // Add archive contents:
    let file = fs::File::open(&path).unwrap();
    let archive_info = match zip::ZipArchive::new(file) {
        Ok(a) => info_for_archive(a).to_string(),
        Err(a) => " is not a Zip archive".to_string(),
    };

    format!("{}{}", info, archive_info)
}

pub fn display_info_for_paths(paths: Vec<String>) {
    for path in paths {
        println!("{}", info_for_path(path));
    }
}

#[cfg(test)]
mod test {
    // Not sure how to test this yet...
    // Mostly integration-test-oriented?
}
