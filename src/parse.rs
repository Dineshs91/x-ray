use std::fs;
use std::path::PathBuf;

use util;
use parser;
use parser::{ItemKind};
use structures::{Root, Package, Module, Class, Function};

/// Check if a given directory is a python package.
fn is_package(dir_path: &PathBuf) -> bool {
    let dirs = fs::read_dir(dir_path).unwrap();
    for dir in dirs {
        let dir_entry = dir.unwrap();

        let file_name = dir_entry.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name == "__init__.py" {
            return true;
        }
    }

    false
}

/// Parse the package and the modules it has.
/// Do this recursively.
fn parse_package(dir_path: &PathBuf) -> Package {
    let package_name = match dir_path.file_name() {
        Some(x) => x.to_str().unwrap_or("").to_string(),
        None => "".to_string()
    };

    let dirs = fs::read_dir(dir_path).unwrap();
    let mut pac_modules: Vec<Module> = Vec::new();
    let mut nested_packages: Vec<Package> = Vec::new();

    for dir in dirs {
        let dir_entry = dir.unwrap();
        let dir_path = dir_entry.path();
        let file_name = dir_entry.file_name();
        let mut file_name = file_name.to_str().unwrap();

        let is_dir: bool = dir_entry.metadata().unwrap().is_dir();

        if is_dir == false {

            if file_name.ends_with(".py") && file_name != "__init__.py" {
                file_name = file_name.split(".").collect::<Vec<_>>()[0];
                pac_modules.push(parse_module(&dir_path, file_name));
            }
        } else {
            let is_py_package = is_package(&dir_path);
            if is_py_package == true {
                let package_res = parse_package(&dir_path);
                nested_packages.push(package_res);
            }
        }
    }

    Package {
        name: package_name,
        packages: nested_packages,
        modules: pac_modules
    }
}

fn parse_module(dir_path: &PathBuf, file_name: &str) -> Module {
    let module_src = util::read_file(dir_path.to_str().unwrap());
    let src_bytes = module_src.as_bytes();

    let parsing_result = parser::parse(src_bytes);
    let mut func_vec: Vec<Function> = Vec::new();
    let mut class_vec: Vec<Class> = Vec::new();

    let mut module_description = None;
    for res in parsing_result {
        match res.node {
            ItemKind::Function{name, description: desc, parameters: params} => {
                func_vec.push(Function {
                    name: name,
                    description: desc,
                    parameters: params
                });
            },
            ItemKind::Class{name, description: desc, methods: mthds} => {
                class_vec.push(Class {
                    name: name,
                    description: desc,
                    methods: mthds
                });
            },
            ItemKind::Module{description:desc} => {
                module_description = desc;
            },
            _ => println!("Found other type in parsing result")
        }
    }
    let module_res = Module {
        name: file_name.to_string(),
        description: module_description,
        functions: func_vec,
        classes: class_vec
    };

    module_res
}

pub fn parse(parse_dir: &str) -> Root {
    let root_name = parse_dir.clone().to_string();
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
            let mut file_name = file_name.to_str().unwrap();
            if file_name.ends_with(".py") && file_name != "__init__.py" {
                file_name = file_name.split(".").collect::<Vec<_>>()[0];
                root_modules.push(parse_module(&dir_path, file_name));
            }
        } else {        
            let is_py_package = is_package(&dir_path);
            if is_py_package == true {
                let package_res = parse_package(&dir_path);
                root_packages.push(package_res);
            }
        } 
    }
    let root_res = Root {
        name: root_name,
        packages: root_packages,
        modules: root_modules
    };

    root_res
}
