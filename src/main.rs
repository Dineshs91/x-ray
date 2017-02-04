extern crate toml;

mod code_generator;

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
        None => println!("Root not found")
    };

    // Get all the functions defined. This will be an array.
    let mut function_values = None;

    match value.get("functions") {
        Some(x) => function_values = Some(x),
        None => println!("functions not found"),
    };

    println!("{:?}", function_values.unwrap());

    // Loop through function values.
    for function_val in function_values.iter() {
        println!("{:?}", function_val);
    }

    println!("{}", code_generator::function_template("display", "This is the display function"));
}
