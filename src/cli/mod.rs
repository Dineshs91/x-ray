use std::env;

use clap::{Arg, App, SubCommand};

const ABOUT: &'static str = "
x-ray generates python code from a configuration file and vice versa.
";

pub enum ConfType {
    Toml,
    Yaml
}

pub struct CliConf {
    pub skip_validations: bool,
    pub conf_file: Option<String>,
    pub conf_type: ConfType,
    pub parse: bool,
    pub parse_dir: Option<String>,
    pub gen_dir: Option<String>,
}

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

pub fn main() -> CliConf {
    let app = App::new("x-ray")
        .version("0.1.0")
        .author("Dineshs91 <dineshspy07@gmail.com>")
        .about(ABOUT)
        .subcommand(SubCommand::with_name("gen")
            .about("generate code from config file")
            .arg(Arg::with_name("skip_validations")
                .short("s")
                .long("skip-validations")
                .help("Skip any python validations"))
            .arg(Arg::with_name("conf_file")
                .short("f")
                .value_name("conf_file")
                .required(true)
                .help("Provide the conf file"))
            .arg(Arg::with_name("conf_type")
                .short("t")
                .value_name("conf_type")
                .required(true)
                .help("Provide the config type (Toml/Yaml)"))
            .arg(Arg::with_name("dir")
                .short("d")
                .value_name("dir")
                .help("Provide the path where the generated code should be put.")))
        .subcommand(SubCommand::with_name("parse")
            .about("parse python source and generate conf file")
            .arg(Arg::with_name("dir")
                .short("d")
                .value_name("dir")
                .required(true)
                .help("Provide the path of python project"))
            .arg(Arg::with_name("conf_file")
                .short("f")
                .value_name("conf_file")
                .required(true)
                .help("Provide the name of the conf file "))
            .arg(Arg::with_name("conf_type")
                .short("t")
                .value_name("conf_type")
                .required(true)
                .help("Provide the config type (Toml/Yaml)")));

    let matches = app.get_matches();

    let mut skip_validations: bool = false;
    let mut conf_file = "";
    let mut conf_type = ConfType::Toml;
    let mut parse_dir = None;
    let mut gen_dir = Some(get_current_directory());

    if let Some(matches) = matches.subcommand_matches("gen") {
        if matches.is_present("skip_validations") {
            println!("Skipping python validations");
            skip_validations = true;
        }

        conf_file = matches.value_of("conf_file").unwrap();
        let conf_type_str = matches.value_of("conf_type").unwrap();

        if conf_type_str == "toml" {
            conf_type = ConfType::Toml;
        } else if conf_type_str == "yaml" {
            conf_type = ConfType::Yaml;
        }

        gen_dir = Some(matches.value_of("dir").unwrap().to_string());
    }

    let parse = match matches.subcommand_matches("parse") {
        Some(_) => true,
        None => false,
    };

    if let Some(matches) = matches.subcommand_matches("parse") {
        parse_dir = Some(matches.value_of("dir").unwrap().to_string());
        conf_file = matches.value_of("conf_file").unwrap();
        let conf_type_str = matches.value_of("conf_type").unwrap();

        if conf_type_str == "toml" {
            conf_type = ConfType::Toml;
        } else if conf_type_str == "yaml" {
            conf_type = ConfType::Yaml;
        }
    }

    let cli_conf: CliConf = CliConf {
        skip_validations: skip_validations,
        conf_file: Some(conf_file.to_string()),
        conf_type: conf_type,
        parse: parse,
        parse_dir: parse_dir,
        gen_dir: gen_dir
    };

    return cli_conf;
}

fn get_current_directory() -> String {
    let cwd = env::current_dir().unwrap();
    cwd.to_str().unwrap().to_string()
}
