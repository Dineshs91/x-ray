#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

mod code_generator;
mod structures;

use std::io::prelude::*;
use std::fs::File;

use structures::{Config};


fn read_toml() -> String {
    let f = File::open("sample.toml");

    let mut s = String::new();

    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("The following error occurred {:?}", error),
    };

    match f.read_to_string(&mut s) {
        Ok(x) => println!("Read size: {}", x),
        Err(error) => panic!("There was an error {:?}", error),
    }

    // return the file content.
    s
}


fn main() {
    let toml_file_content = read_toml();
    let config: Config = toml::from_str(&toml_file_content).unwrap();

    println!("{:?}", config);
}
