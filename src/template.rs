extern crate rustache;

use std::io::Cursor;
use self::rustache::{HashBuilder, Render};
use structures::{Class, Function};


pub fn module_desc_template(description: String) -> String {
    let module_desc_template = r#"
"""
{{module_desc}}
"""
"#;
 
    let mut data = HashBuilder::new();
    data = data.insert("module_desc", description);

    let mut out = Cursor::new(Vec::new());
    match data.render(module_desc_template, &mut out) {
        Ok(_) => {},
        Err(e) => panic!("Error rendering template {:?}", e)
    };

    String::from_utf8(out.into_inner()).unwrap()
}

pub fn function_template(function: Function) -> String {
    let mut func_desc_bool = false;
    let func_desc = match function.description {
        Some(val) => {
            func_desc_bool = true;
            val
        },
        None => String::new()
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
    match data.render(function_template, &mut out) {
        Ok(_) => {},
        Err(e) => panic!("Error rendering template {:?}", e)
    };

    String::from_utf8(out.into_inner()).unwrap()
}

pub fn method_template(method: Function) -> String {
    let method_template = r#"
    def {{ method_name }}({{method_parameters}}):
        {{#method_desc_bool}}"""
        {{ method_desc }}
        """{{/method_desc_bool}}
        pass
"#;
    let mut method_parameters = String::new();
    let mut i:i32 = 0;

    for parameter in method.parameters {
        if i != 0 {
            method_parameters += ", "
        }
        method_parameters += &parameter;
        i += 1;
    }

    let mut method_desc_bool = false;

    let method_desc = match method.description {
        Some(val) => {
            method_desc_bool = true;
            val
        },
        None => String::new(),
    };

    let mut method_data = HashBuilder::new();
    method_data = method_data.insert("method_parameters", method_parameters);
    method_data = method_data.insert("method_name", method.name);
    method_data = method_data.insert("method_desc_bool", method_desc_bool);
    method_data = method_data.insert("method_desc", method_desc);

    let mut method_out = Cursor::new(Vec::new());
    match method_data.render(&method_template, &mut method_out) {
        Ok(_) => {},
        Err(e) => panic!("Error rendering template {:?}", e)
    };

    String::from_utf8(method_out.into_inner()).unwrap()
}

pub fn class_template(class: Class) -> String {
    let mut class_desc_bool = false;
    let class_desc = match class.description {
        Some(val) => {
            class_desc_bool = true;
            val
        },
        None => String::new()
    };

    let mut class_inheritance_bool = false;

    if class.parents.len() > 0 {
        class_inheritance_bool = true;
    }

    let mut class_inheritance = String::new();
    for (index, val) in class.parents.iter().enumerate() {
        if index != 0 {
            class_inheritance += ", "
        }
        class_inheritance += val
    }

    let class_template = r#"
class {{ class_name }}{{#class_inheritance_bool}}(class_inferitance){{/class_inheritance_bool}}:
    {{#class_desc_bool}}"""
    {{ class_desc }}
    """{{/class_desc_bool}}
"#;

    let mut method_template_string = String::new();
    let methods = class.methods;

    for method in methods {
        method_template_string += &method_template(method);
    }

    let mut data = HashBuilder::new();
    data = data.insert("class_name", class.name);
    data = data.insert("class_desc", class_desc);
    data = data.insert("class_desc_bool", class_desc_bool);
    data = data.insert("class_inheritance", class_inheritance);
    data = data.insert("class_inheritance_bool", class_inheritance_bool);

    let mut out = Cursor::new(Vec::new());
    match data.render(class_template, &mut out) {
        Ok(_) => {},
        Err(e) => panic!("Error rendering template {:?}", e)
    };

    // return the filled class template
    String::from_utf8(out.into_inner()).unwrap() + &method_template_string
}

// Unit tests.
#[test]
fn test_function_template() {
    let function = Function {
        name: "display".to_string(),
        description: Some("This is the display function.".to_string()),
        parameters: vec!["self".to_string(), "params".to_string()]
    };

    let function_template_content = function_template(function);
    let expected_function_template_content = r#"
def display(self, params):
    """
    This is the display function.
    """
    pass
"#;

    assert_eq!(function_template_content, expected_function_template_content);
}

#[test]
fn test_class_template() {
    let class = Class {
        name: "Animal".to_string(),
        description: Some("This is the animal class.".to_string()),
        parents: Vec::new(),
        methods: Vec::new()
    };

    let class_template_content = class_template(class);
    let expected_class_template_content = r#"
class Animal:
    """
    This is the animal class.
    """
"#;

    assert_eq!(class_template_content, expected_class_template_content);
}

#[test]
fn test_class_template_with_methods() {
    let mut methods = Vec::new();

    let function = Function {
        name: "display".to_string(),
        description: Some("This is the display function.".to_string()),
        parameters: Vec::new()
    };

    methods.push(function);

    let class = Class {
        name: "Animal".to_string(),
        description: Some("This is the animal class.".to_string()),
        parents: Vec::new(),
        methods: methods
    };

    let class_template_content = class_template(class);

    let expected_class_template_content = r#"
class Animal:
    """
    This is the animal class.
    """

    def display():
        """
        This is the display function.
        """
        pass
"#;

    assert_eq!(class_template_content, expected_class_template_content);
}