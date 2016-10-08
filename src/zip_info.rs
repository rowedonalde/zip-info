extern crate zip;

use std::fs;

fn info_for_path(path: String) -> String {
    let mut info = String::new();
    info = format!("{}", path);

    // Add archive contents:
    let file = fs::File::open(&path).unwrap();
    let mut whole_archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..whole_archive.len() {
        let archive_item = whole_archive.by_index(i).unwrap();
        let item_path = archive_item.name();
        info = format!("{}\n\t{}", info, item_path);
    }

    info
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
