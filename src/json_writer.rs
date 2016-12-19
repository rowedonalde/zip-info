use ::serde;

struct ZipArchiveJsonWriter {

}

#[derive(Serialize, Debug)]
struct ZipObjectJsonWriter {
   compression_type: String,
   original_size: u64,
   compressed_size: u64,
   compression_rate: String,
}
