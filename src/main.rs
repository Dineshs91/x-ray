#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

mod code_generator;
mod structures;

use std::io::prelude::*;
use std::fs::File;

use structures::{Config};
use code_generator::{write_to_file, class_template, function_template};


fn read_toml() -> String {
    let file = File::open("sample.toml");

    let mut file_content = String::new();

    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("The following error occurred {:?}", error),
    };

    match file.read_to_string(&mut file_content) {
        Ok(x) => println!("Read size: {}", x),
        Err(error) => panic!("There was an error {:?}", error),
    }

    // return the file content.
    file_content
}


fn main() {
    let toml_file_content = read_toml();
    let config: Config = toml::from_str(&toml_file_content).unwrap();

    println!("{:?}", config);

    // Root have modules
    // Modules have functions
    let root = config.root;
    
    for module in root.modules {
        let functions = module.functions;
        let ref filename = module.name;

        let classes = module.classes;
        for class in classes {
            let content = class_template(class);
            write_to_file(&filename, &content);
        }

        for function in functions {
            let content = function_template(function);
            //write_to_file(&filename, &content);
        }
    }
}
