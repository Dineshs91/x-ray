use std::path::{Path, PathBuf};
use std::fs;

use toml;
use serde_yaml;

use cli::ConfType;
use util::{read_file, write_to_file, create_package};
use template::{module_desc_template, class_template, function_template};
use structures::{Config, Root, Package, Module, Validate};

fn validate (root: &Root) {
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
}

fn generate_package_src(packages: Vec<Package>, package_path: &Path) {
    for package in packages {
        let package_path = package_path.join(package.name);
        create_package(&package_path);

        generate_package_src(package.packages, &package_path);

        let modules = package.modules;

        generate_module_src(modules, &package_path);
    }
}

fn generate_module_src(modules: Vec<Module>, path: &Path) {
    for module in modules {
        let functions = module.functions;
        let ref filename = module.name;

        let classes = module.classes;
        let mut content = String::new();

        match module.description {
            Some(desc) => {
                content += &module_desc_template(desc);
                write_to_file(&path, &filename, &content);
            },
            None => {}
        };

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

pub fn generate(skip_validations: bool, gen_dir: String, conf_file: &str, conf_type: ConfType) {
    let file_content = read_file(conf_file);
    let config: Config = match conf_type {
        ConfType::Toml => toml::from_str(&file_content).unwrap(),
        ConfType::Yaml => serde_yaml::from_str(&file_content).unwrap()
    };

    // Root have packages
    // Packages have modules. They can have nested packages.
    // Modules have functions
    let root = config.root;

    if !skip_validations {
        validate(&root);
    }

    fs::create_dir_all(&gen_dir).expect("Unable to create the provided generate directory");

    let root_path = PathBuf::from(gen_dir);
    let root_path = root_path.as_path();

    generate_module_src(root.modules, root_path);
    generate_package_src(root.packages, root_path);
}
