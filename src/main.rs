extern crate toml;

use std::io::prelude::*;
use std::fs::File;


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

    let value = toml::Parser::new(&toml_file_content).parse().unwrap();

    // Check if it is root table.
    match value.get("root") {
        Some(x) => println!("{}", x),
        None => println!("None")
    }
}
