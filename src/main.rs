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

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use structures::{Config, Root, Package, Module, Class, Function, Validate};
use template::{class_template, function_template};
use util::{write_to_file, create_package};
use parser::{ItemKind};

fn read_file(filename: &str) -> String {
    let file = File::open(filename);

    let mut file_content = String::new();

    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("The following error occurred {:?}", error),
    };

    match file.read_to_string(&mut file_content) {
        Ok(_) => {},
        Err(error) => panic!("There was an error {:?} reading the file {}", error, filename),
    }

    // return the file content.
    file_content
}

fn validate (root: Root) -> Root {
    for package in &root.packages {
        let ref modules: Vec<Module> = package.modules;

        for module in modules {
            let ref functions = module.functions;
            let ref classes = module.classes;

            for function in functions {
                let is_valid: bool = function.validate_case();

                if !is_valid {
                    panic!("Invalid function name format");
                }
            }

            for class in classes {
                let is_valid: bool = class.validate_case();

                if !is_valid {
                    panic!("Invalid class name format");
                }
            }
        }
    }

    root
}

fn generate(skip_validations: bool, conf_file: String) {
    let toml_file_content = read_file(&conf_file);
    let config: Config = toml::from_str(&toml_file_content).unwrap();

    // Root have packages
    // Packages have modules
    // Modules have functions
    let mut root = config.root;

    if !skip_validations {
        root = validate(root);
    }

    for package in root.packages {
        create_package(&package.name);

        let path = package.name;

        let modules = package.modules;

        for module in modules {
            let functions = module.functions;
            let ref filename = module.name;

            let classes = module.classes;
            let mut content = String::new();

            for class in classes {
                content += &class_template(class);
                write_to_file(&path, &filename, &content);
            }

            for function in functions {
                content += &function_template(function);
            }

            write_to_file(&path, &filename, &content);
        }
    }
}

/// Check if a given directory is a python package.
fn is_package(dir_path: PathBuf) -> (bool, PathBuf) {
    let dir_path_copy = dir_path.clone();
    let dirs = fs::read_dir(dir_path).unwrap();
    for dir in dirs {
        let dir_entry = dir.unwrap();
        let dir_path = dir_entry.path();

        let file_name = dir_entry.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name == "__init__.py" {
            return (true, dir_path);
        }
    }

    (false, dir_path_copy)
}

fn parse(parse_dir: String) {
    // Start from a directory. Can add file support later.
    // Parse all 
    //   1. Individual modules.
    //   2. Packages. -> Modules.
    let root_name = parse_dir.clone();
    let dir_path = PathBuf::from(parse_dir);
    let dirs = fs::read_dir(dir_path).unwrap();
    let mut root_packages: Vec<Package> = Vec::new();
    let mut root_modules: Vec<Module> = Vec::new();

    for dir in dirs {
        let dir_entry = dir.unwrap();
        let dir_path = dir_entry.path();
        let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

        if is_dir == false {
            let file_name = dir_entry.file_name();
            let file_name = file_name.to_str().unwrap();
            if file_name.ends_with(".py") {
                let module_src = read_file(dir_path.to_str().unwrap());
                let src_bytes = module_src.as_bytes();

                let parsing_result = parser::parse(src_bytes).unwrap().1;
                let mut func_vec: Vec<Function> = Vec::new();
                let mut class_vec: Vec<Class> = Vec::new();

                for res in parsing_result {
                    match res.node {
                        ItemKind::Function{name: name, description: desc, parameters: params} => {
                            func_vec.push(Function {
                                name: name,
                                description: desc,
                                parameters: params
                            });
                        },
                        ItemKind::Class{name: name, description: desc, methods: mthds} => {
                            class_vec.push(Class {
                                name: name,
                                description: desc,
                                methods: mthds
                            });
                        },
                        _ => println!("Found other type")
                    }
                }
                let module_res = Module {
                    name: file_name.to_string(),
                    description: None,
                    functions: func_vec,
                    classes: class_vec
                };
                root_modules.push(module_res);
            }
        } else {
            let (is_py_package, dir_path) = is_package(dir_path);
            if is_py_package == true {
                parse_package(dir_path);
            }
        } 
    }
    let root_res = Root {
        name: root_name,
        description: None,
        packages: root_packages,
        modules: root_modules
    };

    println!("{:?}", root_res);
}

/// Parse the package and the modules it has.
/// Do this recursively.
fn parse_package(dir_path: PathBuf) {
    let dirs = fs::read_dir(dir_path).unwrap();

    for dir in dirs {
        let dir_entry = dir.unwrap();
        let dir_path = dir_entry.path();
        let file_name = dir_entry.file_name();
        let file_name = file_name.to_str().unwrap();

        if file_name.ends_with(".py") {
            let module_src = read_file(dir_path.to_str().unwrap());
            let src_bytes = module_src.as_bytes();
            let result = parser::parse(src_bytes);
            println!("Result2 is {:?}", result);
        } else {
            let (is_py_package, dir_path) = is_package(dir_path);
            if is_py_package == true {
                // parse_package(dir_path);
                // TODO: Fix this
                println!("Found cascade packages");
            }
        }
    }
}

fn main() {
    let cli_values = cli::main();
    let skip_validations = cli_values.skip_validations;
    let conf_file = cli_values.conf_file.unwrap();
    let parse_opt = cli_values.parse;
    let parse_dir = cli_values.parse_dir;

    if parse_opt {
        parse(parse_dir.unwrap());
    } else {
        generate(skip_validations, conf_file);
    }
}
