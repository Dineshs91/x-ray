extern crate rustache;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use self::rustache::{HashBuilder, Render};
use structures::Function;

// pub fn templates() -> String {
//     // Return the corresponding template Ex: class, function, method etc.
// }


pub fn function_template(function: Function) -> String {
	// return function template
	let mut func_bool = false;
	// let func_desc = function.description.unwrap();
	let func_desc = match function.description {
		Some(val) => {
			func_bool = true;
			val
		},
		None => {
			func_bool = false;
			String::new()
		},
	};

	let function_template = r#"
def {{func_name}}():
	{{#func_bool}}
    """
    {{func_desc}}
    """
    {{/func_bool}}
    pass
	"#;

	let mut data = HashBuilder::new();
	data = data.insert("func_name", function.name);
	data = data.insert("func_desc", func_desc);
	data = data.insert("func_bool", func_bool);

	let mut out = Cursor::new(Vec::new());
	data.render(function_template, &mut out);

	// return the filled template.
	// TODO: Handle error's
	String::from_utf8(out.into_inner()).unwrap()
}

fn class_template(class_name: &str, class_desc: &str) -> String {
	// return class template
	let class_template = r#"
class {{ class_name }}:
    """
    {{ class_desc }}
    """

    def __init__(self):
        pass
	"#;

	class_template.to_string()
}

// fn method_template() -> String {
// 	// return method template
// }

pub fn write_to_file(filename: &str, content: &str) {
	// Write the python source to file.
	// Args: file name, the content of the file.
	//
	// filename & content will be &str since we won't be manipulating it.
}