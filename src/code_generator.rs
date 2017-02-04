use std::io::prelude::*;
use std::fs::File;


pub fn templates() -> String {
    // Return the corresponding template Ex: class, function, method etc.
}

fn function_template(func_name, func_description) -> String {
	// return function template
}

fn class_template() -> String {
	// return class template
}

fn method_template() -> String {
	// return method template
}

pub fn write_to_file(filename: &str, content: &str) {
	// Write the python source to file.
	// Args: file name, the content of the file.
	//
	// filename & content will be &str since we won't be manipulating it.
}