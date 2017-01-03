use ::serde_json;
use ::zip;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct MultiArchiveJsonWriter {
    archives: HashMap<String, ZipArchiveJsonWriter>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ZipArchiveJsonWriter {
    objects: HashMap<String, ZipObjectJsonWriter>,
}

impl ZipArchiveJsonWriter {
    pub fn new() -> ZipArchiveJsonWriter {
        ZipArchiveJsonWriter { objects: HashMap::new() }
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
        let zip_object_name = String::from("foo.txt");
        let zip_object = get_zip_object();

        {
            let mut zip_archive = ZipArchiveJsonWriter::new();
            zip_archive.objects.insert(zip_object_name, zip_object);

            let zip_archive_serialized = serde_json::to_string(&zip_archive)
                .unwrap();
            println!("zip_archive_serialized: {}", zip_archive_serialized);

            let zip_archive_pretty = serde_json::to_string_pretty(&zip_archive)
                .unwrap();
            println!("zip_archive_pretty: {}", zip_archive_pretty);

            let zip_archive_deserialized: ZipArchiveJsonWriter =
                serde_json::from_str(zip_archive_serialized.as_str()).unwrap();

            let zip_archive_depretty: ZipArchiveJsonWriter =
                serde_json::from_str(zip_archive_serialized.as_str()).unwrap();

            assert_eq!(zip_archive, zip_archive_deserialized);
            assert_eq!(zip_archive, zip_archive_depretty);
        }
    }
}
