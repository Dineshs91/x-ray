mod util;
#[macro_use]
mod mac;

use std;

use nom;

use structures::{Function};

#[derive(Debug, Eq, PartialEq)]
pub struct Item {
    pub node: ItemKind,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ItemKind {
    Import {path: String},
    ImportFrom {module: String, name: String, level: i32},
    Shebang {path: String},
    Module {description: Option<String>},
    Class {name: String, description: Option<String>, parents: Vec<String>, methods: Vec<Function>},
    Function {name: String, description: Option<String>, parameters: Vec<String>},
    Code {code: String}
}

named!(items<Vec<Item>>, many0!(alt!(
    shebang
    |
    item_import
    |
    item_import_from
    |
    item_class
    |
    item_fn
    |
    item_code
)));

named!(item_code<Item>, do_parse!(
    many0!(nom::newline) >>
    code: map_res!(take_until_and_consume!("\n"), std::str::from_utf8) >>
    many0!(nom::newline) >>
    (Item {
        node: ItemKind::Code {
            code: code.to_string()
        }
    })
));

named!(shebang<Item>, do_parse!(
    many0!(nom::newline) >>
    tag!("#!") >>
    many1!(nom::space) >>
    path: map_res!(take_until_and_consume!("\n"), std::str::from_utf8) >>
    (Item {
        node: ItemKind::Shebang {
            path: path.to_string()
        }
    })
));

named!(item_import<Item>, do_parse!(
    many0!(nom::newline) >>
    tag!("import") >>
    many1!(nom::space) >>
    path: map_res!(take_until_and_consume!("\n"), std::str::from_utf8) >>
    (Item {
        node: ItemKind::Import {
            path: path.to_string()
        }
    })
));

named!(item_import_from<Item>, do_parse!(
    many0!(nom::newline) >>
    tag!("from") >>
    many1!(nom::space) >>
    module: map_res!(take_until_and_consume!(" import"), std::str::from_utf8) >>
    many1!(nom::space) >>
    name: map_res!(take_until_and_consume!("\n"), std::str::from_utf8) >>
    (Item {
        node: ItemKind::ImportFrom {
            module: {
                // Just strip the initial dots in the module.
                let mut module_vec = Vec::new();
                let mut init = false;
                for ch in module.chars() {
                    if init == false && ch == '.' {
                        continue
                    } else {
                        init = true;
                        module_vec.push(ch);
                    }
                }
                let module_str: String = module_vec.into_iter().collect();
                module_str
            },
            name: name.to_string(),
            level: {
                let mut level;
                // Level: 0 is absolute import
                if module.starts_with(".") {
                    level = 0;
                    for ch in module.chars() {
                        if ch == '.' {
                            level += 1;
                        } else {
                            break;
                        }
                    }
                } else {
                    level = 0;
                }

                level
            }
        }
    })
));

named!(item_module_doc_string<Item>, do_parse!(
    description: opt!(doc_string) >>
    (Item {
        node: ItemKind::Module {
            description: description
        }
    })
));

/// Use this for parsing class inheritance
/// Example:
///
/// class Dog(Animal):
///    def __init__(self):
///        pass
///
named!(parent<Vec<&[u8]>>, do_parse!(
    tag!("(") >>
    parents: ws!(separated_list!(tag!(","), util::ident)) >>
    tag!(")") >>
    (parents)
));

pub enum ClassBlock {
    Method (Item),
    Code (String)
}

named!(item_class_code<ClassBlock>, do_parse!(
    many0!(nom::newline) >>
    code: map_res!(take_until_and_consume!("\n"), std::str::from_utf8) >>
    (ClassBlock::Code(code.to_string()))
));

named!(item_class_method<ClassBlock>, do_parse!(
    method: call!(item_fn) >>
    (ClassBlock::Method(method))
));

named!(item_class_block<ClassBlock>, alt!(
    item_class_method
    |
    item_class_code
));

named!(item_class<Item>, do_parse!(
    many0!(nom::newline) >>
    start_len: many0!(tag!(" ")) >>
    tag!("class") >>
    many1!(nom::space) >>
    name: map_res!(util::ident, std::str::from_utf8) >>
    parents: opt!(parent) >>
    tag!(":") >>
    description: opt!(doc_string) >>
    opt!(util::emptyline) >>
    opt!(take_until_line_containing_tag!("def")) >>
    class_items: many0_block!(start_len.len(), call!(item_class_block)) >>
    many0!(nom::newline) >>
    (Item {
        node: ItemKind::Class {
            name: name.to_string(),
            description: description,
            parents: {
                let parents = match parents {
                    Some(x) => x,
                    None => Vec::new()
                };

                // Return parents after converting to String.
                parents.iter().map(|x| std::str::from_utf8(x).unwrap().to_string()).collect::<Vec<_>>()
            },
            methods: {
                let mut result = Vec::new();
                let mut methods = Vec::new();
                for class_item in class_items {
                    match class_item {
                        ClassBlock::Method(func) => methods.push(func),
                        ClassBlock::Code(_) => {}
                    };
                }
                for item in methods {
                    match item.node {
                        ItemKind::Function {name, description, parameters} => {
                            result.push(Function {
                                name: name,
                                description: description,
                                parameters: parameters
                            });
                        },
                        _ => {}
                    }
                }

                result
            }
        }
    })
));

named!(item_fn<Item>, do_parse!(
    many0!(nom::newline) >>
    start_len: many0!(tag!(" ")) >>
    decorators: many0!(decorator) >>
    tag!("def") >>
    space: many1!(nom::space) >>
    name: map_res!(util::ident, std::str::from_utf8) >>
    ws!(tag!("(")) >>
    params: ws!(separated_list!(tag!(","), util::func_param)) >>
    opt!(tag!(",")) >>
    ws!(tag!("):")) >>
    opt!(nom::newline) >>
    description: opt!(doc_string) >>
    has_data: has_data!() >>
    cond!(has_data, block!(start_len.len())) >>

    (Item {
        node: ItemKind::Function {
            name: name.to_string(),
            description: description,
            parameters: params.iter().map(|x| std::str::from_utf8(x).unwrap().to_string()).collect::<Vec<_>>()
        }
    })
));

named!(decorator<String>,
    do_parse!(
        tag!("@") >>
        decorator_string: map_res!(take_until_and_consume!("\n"), std::str::from_utf8) >>

        (decorator_string.to_string())
    )
);

named!(doc_string<String>,
    do_parse!(
        opt!(nom::multispace) >>
        doc_string: map_res!(
            alt!(
                delimited!(tag!("\"\"\""), take_until!("\"\"\""), tag!("\"\"\""))
                |
                delimited!(tag!("'''"), take_until!("'''"), tag!("'''"))
            ),
            std::str::from_utf8
        ) >>
        (doc_string.trim().to_string())
    )
);

pub fn parse(source: &[u8]) -> Vec<Item> {
    let mut result: Vec<Item> = Vec::new();

    // Module doc string should be the first statement in the module.
    let item_module_doc_string_result = item_module_doc_string(source).unwrap();
    result.push(item_module_doc_string_result.1);

    // items will parse the entire code. It is a custom nom parser method.
    // Pass the remaining values to the items parser.
    let items_result = items(item_module_doc_string_result.0);

    let items_result = match items_result {
        nom::IResult::Done(_, output) => output,
        nom::IResult::Error(e) => panic!("Unable to parse [error] {}", e),
        nom::IResult::Incomplete(n) => {
            println!("Unable to parse [Incomplete] {:?}", n);
            Vec::new()
        }
    };

    result.extend(items_result);

    result
}

#[test]
fn test_parser_shebang() {
    let content = r#"
#! /usr/bin/env python
"#;

    let actual_result = shebang(content.as_bytes());
    let expected_result = Item {
        node: ItemKind::Shebang {
            path: "/usr/bin/env python".to_string()
        }
    };

    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_import() {
    let content = r#"
import os

"#;
    let actual_result = item_import(content.as_bytes());

    let import_os = Item {
        node: ItemKind::Import {
            path: "os".to_string()
        }
    };

    let expected_result = import_os;

    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_import_from_absolute() {
    let content = r#"
from os import stat

"#;
    let actual_result = item_import_from(content.as_bytes());

    let import_os = Item {
        node: ItemKind::ImportFrom {
            module: "os".to_string(),
            name: "stat".to_string(),
            level: 0
        }
    };

    let expected_result = import_os;

    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_import_from_relative_level_1() {
    let content = r#"
from .os import stat

"#;
    let actual_result = item_import_from(content.as_bytes());

    let import_os = Item {
        node: ItemKind::ImportFrom {
            module: "os".to_string(),
            name: "stat".to_string(),
            level: 1
        }
    };

    let expected_result = import_os;

    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_import_from_relative_level_2() {
    let content = r#"
from ..os.stat import __init__

"#;
    let actual_result = item_import_from(content.as_bytes());

    let import_os = Item {
        node: ItemKind::ImportFrom {
            module: "os.stat".to_string(),
            name: "__init__".to_string(),
            level: 2
        }
    };

    let expected_result = import_os;

    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_multiple_imports_and_function() {
    let content = r#"
import os
import imap


def hello():
    """
    Hello function.
    """
    pass
"#;
    let actual_result = items(content.as_bytes());

    let import_os = Item {
        node: ItemKind::Import {
            path: "os".to_string()
        }
    };

    let import_imap = Item {
        node: ItemKind::Import {
            path: "imap".to_string()
        }
    };

    let hello_function = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("Hello function.".to_string()),
            parameters: Vec::new()
        }
    };

    let expected_result = vec!(import_os, import_imap, hello_function);

    println!("Actual result is {:?}", actual_result);
    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_item_module_doc_string() {
    let module_content = r#"
    """
    This is the module doc string.
    """

    """
    This is not the module doc string.
    """
"#;
    let result = item_module_doc_string(module_content.as_bytes());
    let expected_result = Item {
        node: ItemKind::Module {
            description: Some("This is the module doc string.".to_string())
        }
    };
    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_class() {
    let class_content = r#"
class Animal:
    def __init__(self):
        pass
"#;

    let result = item_class(class_content.as_bytes());

    let method = Function {
        name: "__init__".to_string(),
        description: None,
        parameters: vec!("self".to_string())
    };

    let item_kind = ItemKind::Class {
        name: "Animal".to_string(),
        description: None,
        parents: Vec::new(),
        methods: vec!(method)
    };

    let expected_result = Item {
        node: item_kind
    };

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_class_with_inheritance() {
    let class_content = r#"
class Animal(Object):
    def __init__(self):
        pass
"#;

    let result = item_class(class_content.as_bytes());

    let mut params: Vec<String> = Vec::new();

    params.push("self".to_string());

    let method = Function {
        name: "__init__".to_string(),
        description: None,
        parameters: params
    };

    let item_kind = ItemKind::Class {
        name: "Animal".to_string(),
        description: None,
        parents: vec!("Object".to_string()),
        methods: vec!(method)
    };

    let expected_result = Item {
        node: item_kind
    };

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_class_with_class_variables() {
    let class_content = r#"
class Animal:
    start = True

    def __init__(self):
        pass
"#;

    let result = item_class(class_content.as_bytes());

    let mut params: Vec<String> = Vec::new();

    params.push("self".to_string());

    let method = Function {
        name: "__init__".to_string(),
        description: None,
        parameters: params
    };

    let item_kind = ItemKind::Class {
        name: "Animal".to_string(),
        description: None,
        parents: Vec::new(),
        methods: vec!(method)
    };

    let expected_result = Item {
        node: item_kind
    };

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_class_with_multiple_methods() {
    let class_content = r#"
class Animal:
    """
    Animal class.
    """
    def __init__(self):
        """
        Init method.
        """
        pass

    def hello(args):
        """
        Hello method.
        """
        pass
"#;

    let result = item_class(class_content.as_bytes());

    let mut params: Vec<String> = Vec::new();

    params.push("self".to_string());

    let method1 = Function {
        name: "__init__".to_string(),
        description: Some("Init method.".to_string()),
        parameters: params
    };

    let method2 = Function {
        name: "hello".to_string(),
        description: Some("Hello method.".to_string()),
        parameters: vec!["args".to_string()]
    };

    let item_kind = ItemKind::Class {
        name: "Animal".to_string(),
        description: Some("Animal class.".to_string()),
        parents: Vec::new(),
        methods: vec!(method1, method2)
    };

    let expected_result = Item {
        node: item_kind
    };
    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn() {
    let fn_content = r#"
def hello(args):
    """
    This is the hello function.
    """
    pass
"#;

    let result = item_fn(fn_content.as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn_with_only_doc_strings() {
    let fn_content = r#"
def hello(args):
    """
    This is the hello function.
    """
"#;

    let result = items(fn_content.trim().as_bytes());

    let mut expected_result = Vec::new();
    expected_result.push(Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    });

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn_with_decorator() {
    let fn_content = r#"
@test1
@test2
def hello(args):
    """
    This is the hello function.
    """
    pass
"#;

    let result = item_fn(fn_content.as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    println!("The result is [{:?}]", result);
    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn_arg_ending_with_comma() {
    let fn_content = r#"
def hello(args, ):
    """
    This is the hello function.
    """
    pass
"#;

    let result = item_fn(fn_content.as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn_with_underscore() {
    let fn_content = r#"
def __hello__(args):
    """
    This is the hello function.
    """
    pass
"#;

    let result = item_fn(fn_content.as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "__hello__".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn_with_default_arg_value() {
    let fn_content = r#"
def __hello__(args, display=True):
    """
    This is the hello function.
    """
    pass
"#;

    let result = item_fn(fn_content.as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "__hello__".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string(), "display=True".to_string())
        }
    };
    println!("The result is: {:?}", result);
    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_item_fn_with_multiple_functions() {
    let fns_content = r#"
def __hello__(args):
    """
    This is the hello function.
    """
    print "Hello"

def hello(args):
    """
    Another hello function.
    """
    print "Hello"
"#;

    let result = items(fns_content.as_bytes());

    let mut expected_result = Vec::new();

    let fn1 = Item {
        node: ItemKind::Function {
            name: "__hello__".to_string(),
            description: Some("This is the hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    let fn2 = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("Another hello function.".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    expected_result.push(fn1);
    expected_result.push(fn2);

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_items_with_class_and_function() {
    let content = r#"
class Animal:
    """
    This is the animal class.
    """
    def __init__(self):
        """
        Init method.
        """
        for i in range(0, 12):
            print i
        pass

    def get_animal(self):
        """
        Get the animal instance of this object.
        """
        pass

def display(msg):
    """
    This is the display function.
    """
    pass
"#;
    let init_method = Function {
        name: "__init__".to_string(),
        description: Some("Init method.".to_string()),
        parameters: vec!["self".to_string()]
    };

    let get_animal_method = Function {
        name: "get_animal".to_string(),
        description: Some("Get the animal instance of this object.".to_string()),
        parameters: vec!["self".to_string()]
    };

    let class_item = Item {
        node: ItemKind::Class {
            name: "Animal".to_string(),
            description: Some("This is the animal class.".to_string()),
            parents: Vec::new(),
            methods: vec!(init_method, get_animal_method)
        }
    };
    let mut expected_result = Vec::new();
    expected_result.push(class_item);

    let function_item = Item {
        node: ItemKind::Function {
            name: "display".to_string(),
            description: Some("This is the display function.".to_string()),
            parameters: vec!["msg".to_string()]
        }
    };
    expected_result.push(function_item);

    let actual_result = items(content.as_bytes());
    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_items_class_with_non_sequential_methods() {
    let content = r#"
class Animal:
    """
    This is the animal class.
    """
    def __init__(self):
        """
        Init method.
        """
        for i in range(0, 12):
            print i
        pass

    def get_animal(self):
        """
        Get the animal instance of this object.
        """
        pass
    copy = __copy__
"#;
    let init_method = Function {
        name: "__init__".to_string(),
        description: Some("Init method.".to_string()),
        parameters: vec!["self".to_string()]
    };

    let get_animal_method = Function {
        name: "get_animal".to_string(),
        description: Some("Get the animal instance of this object.".to_string()),
        parameters: vec!["self".to_string()]
    };

    let class_item = Item {
        node: ItemKind::Class {
            name: "Animal".to_string(),
            description: Some("This is the animal class.".to_string()),
            parents: Vec::new(),
            methods: vec!(init_method, get_animal_method)
        }
    };
    let mut expected_result = Vec::new();
    expected_result.push(class_item);

    let actual_result = items(content.as_bytes());
    assert_eq!(actual_result.unwrap().1, expected_result);
}

#[test]
fn test_parser_doc_string() {
    let doc_string_content = r#"
    """
    This is the description string.
    """
    "#;

    let result = doc_string(doc_string_content.as_bytes());

    assert_eq!(result.unwrap().1, "This is the description string.");
}

#[test]
fn test_parser_doc_string_with_double_quotes_in_the_text() {
    let doc_string_content = r#"
    """
    This is the description "string".
    """
    "#;

    let result = doc_string(doc_string_content.as_bytes());

    assert_eq!(result.unwrap().1, "This is the description \"string\".");
}

#[test]
fn test_parser_doc_string_with_single_quotes() {
    let doc_string_content = r#"
    '''
    This is the description "string'.
    '''
    "#;

    let result = doc_string(doc_string_content.as_bytes());

    assert_eq!(result.unwrap().1, "This is the description \"string'.");
}