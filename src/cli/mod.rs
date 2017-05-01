use clap::{Arg, App, SubCommand};

const ABOUT: &'static str = "
x-ray generates python code from a configuration file and vice versa.
";

pub struct CliConf {
    pub skip_validations: bool,
    pub conf_file: Option<String>,
    pub parse: bool,
    pub parse_dir: Option<String>
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
                .help("Provide the conf file")))
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
                .help("Provide the name of the conf file ")));

    let matches = app.get_matches();

    let mut skip_validations: bool = false;
    let mut conf_file = "";
    let mut parse_dir = None;
    if let Some(matches) = matches.subcommand_matches("gen") {
        if matches.is_present("skip_validations") {
            println!("Skipping python validations");
            skip_validations = true;
        }

        conf_file = matches.value_of("conf_file").unwrap();
    }

    let parse = match matches.subcommand_matches("parse") {
        Some(_) => true,
        None => false,
    };

    if let Some(matches) = matches.subcommand_matches("parse") {
        parse_dir = Some(matches.value_of("dir").unwrap().to_string());
        conf_file = matches.value_of("conf_file").unwrap();
    }

    let cli_conf: CliConf = CliConf {
        skip_validations: skip_validations,
        conf_file: Some(conf_file.to_string()),
        parse: parse,
        parse_dir: parse_dir
    };

    return cli_conf;
}
