use ::glob;
use ::serde_json;
use ::zip;
use std::collections::HashMap;
use std::fs;

use zip_info::StatArgs;

pub fn zips_to_json(file_paths: &[&str], exclude: &str, stat_args: &StatArgs) -> String {
    zips_to_json_with_printer(file_paths, exclude, stat_args, serde_json::to_string)
}

pub fn zips_to_json_pretty(file_paths: &[&str], exclude: &str, stat_args: &StatArgs) -> String {
    zips_to_json_with_printer(file_paths, exclude, stat_args, serde_json::to_string_pretty)
}

fn zips_to_json_with_printer<F>(file_paths: &[&str], exclude: &str, stat_args: &StatArgs, printer: F) -> String 
    where F: Fn(&MultiArchiveJsonWriter) -> serde_json::Result<String> {
    let multi_archive = MultiArchiveJsonWriter::from(file_paths, exclude, stat_args);
    printer(&multi_archive).unwrap()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MultiArchiveJsonWriter {
    archives: HashMap<String, ZipArchiveJsonWriter>,
}

impl MultiArchiveJsonWriter {
    /// Construct MultiArchiveJsonWriter with empty map of archives:
    pub fn new() -> MultiArchiveJsonWriter {
        MultiArchiveJsonWriter { archives: HashMap::new() }
    }

    /// Create and fill MultiArchiveJsonWriter representing
    /// zero to many .zip files:
    pub fn from(
        file_paths: &[&str], exclude: &str, stat_args: &StatArgs) -> MultiArchiveJsonWriter {
        let mut multi_archive = MultiArchiveJsonWriter::new();

        for path in file_paths {
            multi_archive.archives.insert(
                String::from(*path),
                ZipArchiveJsonWriter::from(*path, exclude, stat_args),
            );
        }

        multi_archive
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ZipArchiveJsonWriter {
    objects: HashMap<String, ZipObjectJsonWriter>,
}

impl ZipArchiveJsonWriter {
    /// Construct ZipArchiveJsonWriter with empty map of objects:
    pub fn new() -> ZipArchiveJsonWriter {
        ZipArchiveJsonWriter { objects: HashMap::new() }
    }

    /// Create and fill ZipArchiveJsonWriter representing a
    /// .zip file:
    pub fn from(file_path: &str, exclude: &str, stat_args: &StatArgs) -> ZipArchiveJsonWriter {
        let mut archive_writer = ZipArchiveJsonWriter::new();
        let exclude_pattern = glob::Pattern::new(exclude).unwrap();

        let file = fs::File::open(file_path).unwrap();
        let mut whole_archive = zip::ZipArchive::new(file).unwrap();

        // Loop through the objects within the archive (.zip file)...
        for i in 0..whole_archive.len() {
            let zip_object = whole_archive.by_index(i).unwrap();

            // ...and if it isn't excluded, add the object to
            // the HashMap in the archive writer:
            if !exclude_pattern.matches(zip_object.name()) {
                archive_writer.objects.insert(
                    String::from(zip_object.name()),
                    // I wanted to prevent the evaluation of as many of these parameters at this level as possible
                    // but it seems that because compression rate depends on both .size() and .compressed_size(), we
                    // need both to be passed in to calculate compression_rate if needed. (No way to encode this 
                    // dependency here without dependent types.)
                    ZipObjectJsonWriter::new(
                        if stat_args.flag_compression_type { Some(zip_object.compression()) } else { None },
                        zip_object.size(),
                        zip_object.compressed_size(),
                        stat_args,
                    ),
                );
            }
        }

        archive_writer
    }
}

// We are going to use Option types here to denote that not
// all of these are going to be calculated, based on certain
// command-line flags given to this program.
//
// TODO: Figure out how to get the JSON serializer to dump fields
// that are set to null, if this is something that we want in the
// feature.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ZipObjectJsonWriter {
    compression_type: Option<String>,
    original_size: Option<u64>,
    compressed_size: Option<u64>,
    compression_rate: Option<String>,
}

impl ZipObjectJsonWriter {
    pub fn new(
        compression_type: Option<zip::CompressionMethod>,
        original_size: u64,
        compressed_size: u64,
        stat_args: &StatArgs,
    ) -> ZipObjectJsonWriter {
        let compression_rate =
            if stat_args.flag_compression_rate {
                Some(match original_size {
                    0 => 0.0 as f64,
                    _ => {
                        (original_size as f64 - compressed_size as f64)
                        / original_size as f64
                    },
                })
            }
            else {
                None
            };

        ZipObjectJsonWriter {
            compression_type: compression_type.and_then(|v| Some(format!("{}", v))),
            original_size: if stat_args.flag_original_size { Some(original_size) } else { None },
            compressed_size: if stat_args.flag_compressed_size { Some(compressed_size) } else { None },
            compression_rate: compression_rate.and_then(|v| Some(format!("{:.*}%", 2, v * 100.0))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_zip_object() -> ZipObjectJsonWriter {
        ZipObjectJsonWriter {
            compression_type: Some(format!("{}", zip::CompressionMethod::Deflated)),
            original_size: Some(100),
            compressed_size: Some(50),
            compression_rate: Some(String::from("50%")),
        }
    }

    fn get_zip_archive() -> ZipArchiveJsonWriter {
        let mut zip_archive = ZipArchiveJsonWriter::new();
        zip_archive.objects.insert(String::from("foo.txt"), get_zip_object());
        zip_archive
    }

    #[test]
    fn test_new_zip_object_calculates_percentages() {
        let zip_object = ZipObjectJsonWriter::new(
            Some(zip::CompressionMethod::Deflated),
            100,
            50,
            &StatArgs::default(),
        );

        assert_eq!("50.00%", zip_object.compression_rate.unwrap_or_default());

        let zip_object_empty = ZipObjectJsonWriter::new(
            Some(zip::CompressionMethod::Stored),
            0,
            0,
            &StatArgs::default(),
        );

        assert_eq!("0.00%", zip_object_empty.compression_rate.unwrap_or_default());

        let zip_object_grew = ZipObjectJsonWriter::new(
            Some(zip::CompressionMethod::Deflated),
            100,
            150,
            &StatArgs::default(),
        );

        assert_eq!("-50.00%", zip_object_grew.compression_rate.unwrap_or_default());
    }

    #[test]
    fn test_serialize_object_json_writer() {
        let zip_object = get_zip_object();

        let zip_object_serialized = serde_json::to_string(&zip_object)
            .unwrap();

        let zip_object_pretty = serde_json::
            to_string_pretty(&zip_object).unwrap();

        let zip_object_deserialized: ZipObjectJsonWriter =
            serde_json::from_str(zip_object_serialized.as_str()).unwrap();

        let zip_object_depretty: ZipObjectJsonWriter =
            serde_json::from_str(zip_object_pretty.as_str()).unwrap();

        assert_eq!(zip_object, zip_object_deserialized);
        assert_eq!(zip_object, zip_object_depretty);
    }

    #[test]
    fn test_new_archive_has_empty_map_of_zip_objects() {
        let zip_archive = ZipArchiveJsonWriter::new();
        let empty_hashmap: HashMap<String, ZipObjectJsonWriter> =
            HashMap::new();

        assert_eq!(empty_hashmap, zip_archive.objects);
    }

    #[test]
    fn test_serialize_archive_json_writer() {
        let zip_archive = get_zip_archive();

        let zip_archive_serialized = serde_json::to_string(&zip_archive)
            .unwrap();

        let zip_archive_pretty = serde_json::to_string_pretty(&zip_archive)
            .unwrap();

        let zip_archive_deserialized: ZipArchiveJsonWriter =
            serde_json::from_str(zip_archive_serialized.as_str()).unwrap();

        let zip_archive_depretty: ZipArchiveJsonWriter =
            serde_json::from_str(zip_archive_pretty.as_str()).unwrap();

        assert_eq!(zip_archive, zip_archive_deserialized);
        assert_eq!(zip_archive, zip_archive_depretty);
    }

    #[test]
    fn test_new_multi_archive_has_empty_map_of_zip_archives() {
        let multi_archive = MultiArchiveJsonWriter::new();
        let empty_hashmap: HashMap<String, ZipArchiveJsonWriter> =
            HashMap::new();

        assert_eq!(empty_hashmap, multi_archive.archives);
    }

    #[test]
    fn test_serialize_multi_archive_json_writer() {
        let zip_archive = get_zip_archive();
        let mut multi_archive = MultiArchiveJsonWriter::new();

        multi_archive.archives.insert(String::from("bar.zip"), zip_archive);

        let multi_archive_serialized = serde_json::to_string(&multi_archive)
            .unwrap();

        let multi_archive_pretty = serde_json::to_string_pretty(&multi_archive)
            .unwrap();

        let multi_archive_deserialized: MultiArchiveJsonWriter =
            serde_json::from_str(multi_archive_serialized.as_str()).unwrap();

        let multi_archive_depretty: MultiArchiveJsonWriter =
            serde_json::from_str(multi_archive_pretty.as_str()).unwrap();

        assert_eq!(multi_archive, multi_archive_deserialized);
        assert_eq!(multi_archive, multi_archive_depretty);
    }
}
