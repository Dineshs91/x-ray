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



pub fn gen(gen_dir: String, conf_file: String) {
    gen::generate(false, gen_dir, conf_file);
}

pub fn parse(parse_dir: String, conf_file: String) {
    let root_res = parse::parse(parse_dir);
    util::write_to_config(conf_file, root_res);
}
