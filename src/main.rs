#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
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

use cli::ConfType;

fn main() {
    let cli_values = cli::main();
    let skip_validations = cli_values.skip_validations;
    let conf_file = cli_values.conf_file.unwrap();
    let conf_type = cli_values.conf_type;
    let parse_opt = cli_values.parse;
    let parse_dir = cli_values.parse_dir;
    let gen_dir = cli_values.gen_dir;

    if parse_opt {
        let root_res = parse::parse(&parse_dir.unwrap());

        let res: String = match conf_type {
            ConfType::Toml => util::get_toml_result(root_res),
            ConfType::Yaml => util::get_yaml_result(root_res)
        };

        util::write_to_config(&conf_file, res);
    } else {
        gen::generate(skip_validations, gen_dir.unwrap(), &conf_file, conf_type);
    }
}
