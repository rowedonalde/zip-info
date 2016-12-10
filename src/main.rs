extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

mod zip_info;
mod flat_writer;

use zip_info::WriteZipInfo;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: zi <path> ...
       zi --help

zi presents information about Zip archives.

Common options:
    -h, --help  Show this usage message
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    //zip_info::display_info_for_paths(args.arg_path);

    for path in args.arg_path {
        let mut wr = flat_writer::ZipInfoFlatWriter::new(path);

        println!("{}", wr.write_zip_info());
    }
}
