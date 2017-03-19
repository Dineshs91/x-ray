extern crate rustache;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use self::rustache::{HashBuilder, Render};
use structures::{Class, Function};


pub fn function_template(function: Function) -> String {
	// return function template
	let mut func_desc_bool = false;
	let func_desc = match function.description {
		Some(val) => {
			func_desc_bool = true;
			val
		},
		None => {
			func_desc_bool = false;
			String::new()
		},
	};

	let function_template = r#"
def {{func_name}}():
    {{#func_desc_bool}}"""
    {{func_desc}}
    """{{/func_desc_bool}}
    pass
	"#;

	let mut data = HashBuilder::new();
	data = data.insert("func_name", function.name);
	data = data.insert("func_desc", func_desc);
	data = data.insert("func_desc_bool", func_desc_bool);

	let mut out = Cursor::new(Vec::new());
	data.render(function_template, &mut out);

	// return the filled template.
	// TODO: Handle error's
	String::from_utf8(out.into_inner()).unwrap()
}

pub fn class_template(class: Class) -> String {
	let mut class_desc_bool = false;
	let class_desc = match class.description {
		Some(val) => {
			class_desc_bool = true;
			val
		},
		None => {
			class_desc_bool = false;
			String::new()
		}
	};

	let class_template = r#"
class {{ class_name }}:
    {{#class_desc_bool}}"""
    {{ class_desc }}
    """{{/class_desc_bool}}
    def __init__(self):
        pass
	"#;

	// TODO: iterate through methods of the class.

	let mut data = HashBuilder::new();
	data = data.insert("class_name", class.name);
	data = data.insert("class_desc", class_desc);
	data = data.insert("class_desc_bool", class_desc_bool);

	let mut out = Cursor::new(Vec::new());
	data.render(class_template, &mut out);

	// return the filled class template
	String::from_utf8(out.into_inner()).unwrap()
}

// fn method_template() -> String {
// 	// return method template
// }

pub fn write_to_file(filename: &str, content: &str) {
	// Write the python source to file.
	// Args: file name, the content of the file.
	//
	// filename & content will be &str since we won't be manipulating it.
	let filename_extension = ".py";
	let filename = filename.to_string() + filename_extension;

	let path = Path::new(&filename);
	let mut file = match File::create(&path) {
		Err(e) => panic!("Error occurred while trying to create file {}", e),
		Ok(file) => file,
	};

	match file.write_all(content.as_bytes()) {
		Err(e) => println!("Error occurred while trying to write to file {}", e),
		Ok(_) => println!("Successfully written content to a file"),
	}
}
