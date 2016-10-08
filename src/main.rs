extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: zi <path>
       zi --help

zi presents information about Zip archives.

Common options:
    -h, --help  Show this usage message
";

#[derive(Debug, RustcDecodable)]
struct Args {}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
}
