#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate regex;
extern crate clap;
#[macro_use]
extern crate nom;

mod template;
mod structures;
mod util;
mod cli;
mod parser;
mod parse;
mod gen;

fn main() {
    let cli_values = cli::main();
    let skip_validations = cli_values.skip_validations;
    let conf_file = cli_values.conf_file.unwrap();
    let parse_opt = cli_values.parse;
    let parse_dir = cli_values.parse_dir;
    let gen_dir = cli_values.gen_dir;

    if parse_opt {
        let root_res = parse::parse(parse_dir.unwrap());
        util::write_to_config(conf_file, root_res);
    } else {
        gen::generate(skip_validations, gen_dir.unwrap(), conf_file);
    }
}
