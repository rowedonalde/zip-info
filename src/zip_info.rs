pub trait WriteZipInfo {
    fn write_zip_info(&mut self, exclude: &str, stat_args: &StatArgs) -> String;
}

// Struct for our command-line arguments
#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_path: Vec<String>,
    pub flag_json: bool,
    pub flag_pretty_json: bool,
    pub flag_exclude: String,
    pub flag_compression_type: bool,
    pub flag_original_size: bool,
    pub flag_compressed_size: bool,
    pub flag_compression_rate: bool,
}

// We will need to pass this down in the main thread, to the individual printers,
// and there is no point in passing down the whole Args struct, in the spirit of encapsulation.
#[derive(Debug)]
pub struct StatArgs {
    pub flag_compression_type: bool,
    pub flag_original_size: bool,
    pub flag_compressed_size: bool,
    pub flag_compression_rate: bool,
}

impl StatArgs {
    pub fn new(args: &Args) -> StatArgs {
        let mut stat_args = StatArgs {
            flag_compression_type: args.flag_compression_type,
            flag_original_size: args.flag_original_size,
            flag_compressed_size: args.flag_compressed_size,
            flag_compression_rate: args.flag_compression_rate,
        };

        // If none of these are explicitly specified, we print all of them.
        if stat_args.is_all_false() {
            stat_args = StatArgs::default();
        }

        stat_args
    }

    pub fn default() -> StatArgs {
        StatArgs {
            flag_compression_type: true,
            flag_original_size: true,
            flag_compressed_size: true,
            flag_compression_rate: true,
        }
    }

    fn is_all_false(&self) -> bool {
        !(self.flag_compression_type || self.flag_original_size || self.flag_compressed_size || self.flag_compression_rate)
    }
}

#[cfg(test)]
mod test {
    // Not sure how to test this yet...
    // Mostly integration-test-oriented?
}
