extern crate rustache;

use std::io::Cursor;
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
def {{func_name}}({{parameters}}):
    {{#func_desc_bool}}"""
    {{func_desc}}
    """{{/func_desc_bool}}
    pass
	"#;

	let mut function_parameters = String::new();
	let mut i:i32 = 0;

	for parameter in function.parameters {
		if i != 0 {
			function_parameters += ", "
		}
		function_parameters += &parameter;
		i += 1;
	}

	let mut data = HashBuilder::new();
	data = data.insert("parameters", function_parameters);
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

	let method_template = r#"
	def {{ method_name }}(self, ):
		{{#method_desc_bool}}"""
		{{ method_desc }}
		"""{{/method_desc_bool}}
        pass
	"#;

	let mut method_template_string = String::new();
	let methods = class.methods;

	for method in methods {
		let method_desc_bool = false;

		let method_desc = match method.description {
			Some(val) => val,
			None => String::new(),
		};

		let mut method_data = HashBuilder::new();
		method_data = method_data.insert("method_name", method.name);
		method_data = method_data.insert("method_desc_bool", true);
		method_data = method_data.insert("method_desc", method_desc);

		let mut method_out = Cursor::new(Vec::new());
		method_data.render(&method_template, &mut method_out);

		method_template_string += &String::from_utf8(method_out.into_inner()).unwrap();
	}

	let mut data = HashBuilder::new();
	data = data.insert("class_name", class.name);
	data = data.insert("class_desc", class_desc);
	data = data.insert("class_desc_bool", class_desc_bool);

	let mut out = Cursor::new(Vec::new());
	data.render(class_template, &mut out);

	// return the filled class template
	String::from_utf8(out.into_inner()).unwrap() + &method_template_string
}
