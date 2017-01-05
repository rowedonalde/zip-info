use ::serde_json;
use ::zip;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MultiArchiveJsonWriter {
    archives: HashMap<String, ZipArchiveJsonWriter>,
}

impl MultiArchiveJsonWriter {
    pub fn new() -> MultiArchiveJsonWriter {
        MultiArchiveJsonWriter { archives: HashMap::new() }
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
    pub fn from(file_path: &str) -> ZipArchiveJsonWriter {
        //let whole_archive = zip::ZipArchive::new(file_path).unwrap();
        // Default result:
        ZipArchiveJsonWriter::new()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ZipObjectJsonWriter {
    compression_type: String,
    original_size: u64,
    compressed_size: u64,
    compression_rate: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_zip_object() -> ZipObjectJsonWriter {
        ZipObjectJsonWriter {
            compression_type: format!("{}", zip::CompressionMethod::Deflated),
            original_size: 100,
            compressed_size: 50,
            compression_rate: String::from("50%"),
        }
    }

    fn get_zip_archive() -> ZipArchiveJsonWriter {
        let mut zip_archive = ZipArchiveJsonWriter::new();
        zip_archive.objects.insert(String::from("foo.txt"), get_zip_object());
        zip_archive
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
