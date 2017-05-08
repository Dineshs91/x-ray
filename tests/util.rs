use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> String {
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