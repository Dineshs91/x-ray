#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate toml;
extern crate regex;
extern crate clap;
#[macro_use]
extern crate nom;

pub mod util;
pub mod cli;
mod template;
mod structures;
mod parser;
mod parse;
mod gen;

use cli::ConfType;


pub fn gen(gen_dir: String, conf_file: &str, conf_type: ConfType) {
    gen::generate(false, gen_dir, conf_file, conf_type);
}

pub fn parse(parse_dir: &str, conf_type: ConfType) -> String {
    let root_res = parse::parse(&parse_dir);

    let res: String = match conf_type {
        ConfType::Toml => util::get_toml_result(root_res),
        ConfType::Yaml => util::get_yaml_result(root_res)
    };

    res
}
