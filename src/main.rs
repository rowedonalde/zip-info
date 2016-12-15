extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

mod zip_info;
mod flat_writer;

use zip_info::WriteZipInfo;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: zi [--exclude=<glob>] <path> ...
       zi --help

zi presents information about Zip archives.

Common options:
    -h, --help        Show this usage message.
    --exclude=<glob>  Ignore objects in the archives whose name
                      is like this glob pattern.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: Vec<String>,
    flag_exclude: String
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    for path in args.arg_path {
        let mut wr = flat_writer::ZipInfoFlatWriter::new(path);

        println!("{}", wr.write_zip_info(args.flag_exclude.as_str()));
    }
}
