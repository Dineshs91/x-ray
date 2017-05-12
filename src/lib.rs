#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate regex;
extern crate clap;
#[macro_use]
extern crate nom;

pub mod util;
mod template;
mod structures;
mod cli;
mod parser;
mod parse;
mod gen;


pub fn gen(gen_dir: String, conf_file: &str) {
    gen::generate(false, gen_dir, conf_file);
}

pub fn parse(parse_dir: &str) -> String {
    let root_res = parse::parse(&parse_dir);
    let toml_res: String = util::get_toml_result(root_res);

    toml_res
}
