mod util;

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
    Class {name: String, description: Option<String>, methods: Vec<Function>},
    Function {name: String, description: Option<String>, parameters: Vec<String>},
}

named!(items<Vec<Item>>, many0!(alt!(
    item_class
    |
    item_fn
)));

named!(item_class<Item>, do_parse!(
    many0!(nom::newline) >>
    tag!("class") >>
    many1!(nom::space) >>
    name: map_res!(nom::alpha, std::str::from_utf8) >>
    tag!(":") >>
    opt!(nom::newline) >>
    description: opt!(doc_string) >>
    opt!(nom::newline) >>
    methods: many0!(item_fn) >>
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

macro_rules! block (
    ($i:expr, $len:expr) => (
        {
            use nom::InputIter;
            use nom::Slice;
            use nom::AsChar;
            let input = $i;

            let cnt = $len as usize;
            let mut res: nom::IResult<_,_> = nom::IResult::Incomplete(nom::Needed::Size(cnt));

            // 1. Get the initial indentation level.
            // 2. Consume everything until the indentation level drops.

            // Variables for subsequent indent tracking.
            let mut start = false;
            let mut indent = 0;
            for (idx, item) in input.iter_indices() {
                res = nom::IResult::Done(input.slice(idx + 1..), input.slice(0..idx + 1));

                // 2. consume everything until the indent level changes.
                if start == false && item.as_char() == '\n' {
                    start = true;
                } else if start == true && item.as_char() == ' ' {
                    indent += 1;
                } else if start == true && item.as_char() == '\n' {
                    start = true;
                    indent = 0;
                } else if start == true && item.as_char() != ' ' {
                    start = false;
                    if indent == cnt {
                        res = nom::IResult::Done(input.slice(idx..), input.slice(0..idx));
                        break;
                        // If indent > indent_level; I should recurse ??
                    } else {
                        indent = 0;
                    }
                }
            };

            res
        }
    );
);

named!(item_fn<Item>, do_parse!(
    start_len: many0!(tag!(" ")) >>
    tag!("def") >>
    space: many1!(nom::space) >>
    name: map_res!(util::ident, std::str::from_utf8) >>
    tag!("(") >>
    params: ws!(separated_list!(tag!(","), nom::alpha)) >>
    tag!("):") >>
    opt!(nom::newline) >>
    description: opt!(ws!(doc_string)) >>
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
        doc_string: map_res!(ws!(delimited!(tag!("\"\"\""), is_not!("\"\"\""), tag!("\"\"\""))), std::str::from_utf8) >>
        (doc_string.to_string())
    )
);

pub fn parse(source: String) {
    let result = items(source.trim().as_bytes());

    println!("The result is {:?}", result);
}

#[test]
fn test_parser_class() {
    let class_content = r#"
class Animal:
    def __init__(self):
        pass
"#;

    let result = item_class(class_content.trim().as_bytes());

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
    def __init__(self):
        pass

    def hello(args):
        pass
"#;

    let result = item_class(class_content.as_bytes());

    let mut params: Vec<String> = Vec::new();

    params.push("self".to_string());

    let method1 = Function {
        name: "__init__".to_string(),
        description: None,
        parameters: params
    };

    let method2 = Function {
        name: "hello".to_string(),
        description: None,
        parameters: vec!["args".to_string()]
    };

    let item_kind = ItemKind::Class {
        name: "Animal".to_string(),
        description: None,
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

    let result = item_fn(fn_content.trim().as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("This is the hello function.\n    ".to_string()),
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

    let result = item_fn(fn_content.trim().as_bytes());

    let expected_result = Item {
        node: ItemKind::Function {
            name: "__hello__".to_string(),
            description: Some("This is the hello function.\n    ".to_string()),
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

    let result = items(fns_content.trim().as_bytes());

    let mut expected_result = Vec::new();

    let fn1 = Item {
        node: ItemKind::Function {
            name: "__hello__".to_string(),
            description: Some("This is the hello function.\n    ".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    let fn2 = Item {
        node: ItemKind::Function {
            name: "hello".to_string(),
            description: Some("Another hello function.\n    ".to_string()),
            parameters: vec!("args".to_string())
        }
    };

    expected_result.push(fn1);
    expected_result.push(fn2);

    assert_eq!(result.unwrap().1, expected_result);
}

#[test]
fn test_parser_doc_string() {
    let doc_string_content = r#"
    """
    This is the description string.
    """
    "#;

    let result = doc_string(doc_string_content.trim().as_bytes());

    assert_eq!(result.unwrap().1.trim(), "This is the description string.\n".trim());
}
