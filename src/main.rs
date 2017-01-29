extern crate toml;

use std::io::prelude::*;
use std::fs::File;



fn read_toml() -> String {
    let mut f = File::open("sample.toml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    // return the file content.
    s
}


fn main() {
    let toml_file_content = read_toml();
    println!("{}", toml_file_content);

    let value = toml::Parser::new(&toml_file_content).parse().unwrap();

    // Check if it is root table.
    match value.get("root") {
        Some(x) => println!("{}", x),
        None => println!("None")
    }
}
