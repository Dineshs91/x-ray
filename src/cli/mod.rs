use std::collections::BTreeMap;
use clap::{Arg, App};

const ABOUT: &'static str = "
x-ray generates python code from a configuration file and vice versa.
";

// There are 2 main parts
//   1. Code generation
//   2. Generate config from code. [TBD]
// x-ray generate
//
// Arguments
//   - Skip validations
//   - Verbose
//   - Help
//   - About

pub fn main() -> BTreeMap<&'static str, bool> {
    let mut cli_config = BTreeMap::new();

    let app = App::new("x-ray")
        .version("0.1.0")
        .author("Dineshs91 <dineshspy07@gmail.com>")
        .about(ABOUT)
        .arg(Arg::with_name("skip_validations")
            .short("s")
            .long("skip-validations")
            .help("Skip any python validations"));

    let matches = app.get_matches();
    let mut skip_validations: bool = false;
    if matches.is_present("skip_validations") {
        println!("Skipping python validations");
        skip_validations = true;
    }

    cli_config.insert("skip_validations", skip_validations);

    cli_config
}
