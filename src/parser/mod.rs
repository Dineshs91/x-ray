mod util;
#[macro_use]
mod mac;

use std;

use nom;

use structures::{Module, Class, Function};

#[derive(Debug, Eq, PartialEq)]
struct Item {
    pub node: ItemKind,
}

#[derive(Debug, Eq, PartialEq)]
enum ItemKind {
    // Module {name: String, description: Option<String>, functions: Vec<Function>, classes: Vec<Class>},
    Import {path: String},
    Shebang {path: String},
    Class {name: String, description: Option<String>, methods: Vec<Function>},
    Function {name: String, description: Option<String>, parameters: Vec<String>}
}

named!(items<Vec<Item>>, many0!(alt!(
    shebang
    |
    item_import
    |
    item_class
    |
    item_fn
)));

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

named!(parent<String>, do_parse!(
    tag!("(") >>
    parent: map_res!(take_until!(")"), std::str::from_utf8) >>
    tag!(")") >>
    (parent.to_string())
));

named!(item_class<Item>, do_parse!(
    many0!(nom::newline) >>
    start_len: many0!(tag!(" ")) >>
    tag!("class") >>
    many1!(nom::space) >>
    name: map_res!(nom::alpha, std::str::from_utf8) >>
    opt!(parent) >>
    tag!(":") >>
    opt!(nom::newline) >>
    description: opt!(doc_string) >>
    methods: many0_block!(start_len.len(), item_fn) >>
    (Item {
        node: ItemKind::Class {
            name: name.to_string(),
            description: description,
            methods: {
                let mut result = Vec::new();

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
    tag!("def") >>
    space: many1!(nom::space) >>
    name: map_res!(util::ident, std::str::from_utf8) >>
    tag!("(") >>
    params: ws!(separated_list!(tag!(","), nom::alpha)) >>
    tag!("):") >>
    opt!(nom::newline) >>
    description: opt!(doc_string) >>
    block!(start_len.len()) >>

    (Item {
        node: ItemKind::Function {
            name: name.to_string(),
            description: description,
            parameters: params.iter().map(|x| std::str::from_utf8(x).unwrap().to_string()).collect::<Vec<_>>()
        }
    })
));

named!(doc_string<String>,
    do_parse!(
        opt!(nom::multispace) >>
        doc_string: map_res!(delimited!(tag!("\"\"\""), is_not!("\"\"\""), tag!("\"\"\"")), std::str::from_utf8) >>
        (doc_string.trim().to_string())
    )
);

pub fn parse(source: String) {
    let result = items(source.as_bytes());

    println!("The result is {:?}", result);
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

    println!("Actual result is {:?}", actual_result);
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
fn test_parser_class() {
    let class_content = r#"
class Animal:
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
fn test_parser_doc_string() {
    let doc_string_content = r#"
    """
    This is the description string.
    """
    "#;

    let result = doc_string(doc_string_content.as_bytes());

    assert_eq!(result.unwrap().1, "This is the description string.");
}
