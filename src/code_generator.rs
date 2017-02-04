extern crate rustache;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use self::rustache::{HashBuilder, Render};


pub fn templates() -> String {
    // Return the corresponding template Ex: class, function, method etc.
}

pub fn function_template(func_name: &str, func_description: &str) -> String {
	// return function template
	let function_template = r#"
def {{ func_name }}():
    """
    {{ func_description }}
    """
    pass
	"#;

	let mut data = HashBuilder::new();
	data = data.insert("func_name", func_name);
	data = data.insert("func_description", func_description);

	let mut out = Cursor::new(Vec::new());
	data.render(function_template, &mut out);

	// return the filled template.
	String::from_utf8(out.into_inner()).unwrap()
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