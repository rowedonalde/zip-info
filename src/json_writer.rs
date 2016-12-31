use ::serde_json;
use ::zip;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct MultiArchiveJsonWriter<'a> {
    archives: HashMap<&'a str, &'a ZipArchiveJsonWriter<'a>>,
}

#[derive(Serialize, Debug)]
struct ZipArchiveJsonWriter<'a> {
    objects: HashMap<&'a str, &'a ZipObjectJsonWriter>,
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

    #[test]
    fn test_serialize_json_writer() {
        let zip_object = ZipObjectJsonWriter {
            compression_type: format!("{}", zip::CompressionMethod::Deflated),
            original_size: 100,
            compressed_size: 50,
            compression_rate: String::from("50%"),
        };

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
}
