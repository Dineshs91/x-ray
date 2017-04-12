use std;

use nom;
use structures::{Module, Class, Function};

#[derive(Debug)]
struct Item {
    pub node: ItemKind,
}

#[derive(Debug)]
enum ItemKind {
    Module {name: String, description: Option<String>, functions: Vec<Function>, classes: Vec<Class>},
    Class {name: String, description: Option<String>, methods: Vec<Function>},
    Function {name: String, description: Option<String>, parameters: Vec<String>},
}

named!(item<Item>, alt!(
    item_class
    |
    item_fn
));

named!(item_class<Item>, do_parse!(
    tag!("class") >>
    many1!(nom::space) >>
    name: map_res!(nom::alpha, std::str::from_utf8) >>
    description: opt!(doc_string) >>
    methods: many0!(item_fn) >>
    (
        Item {
            node: ItemKind::Class{
                name: name.to_string(),
                description: description,
                methods: Vec::new()
            }
        }
    )
));

named!(item_fn<Item>, do_parse!(
    tag!("def") >>
    many1!(nom::space) >>
    name: map_res!(nom::alpha, std::str::from_utf8) >>
    tag!("(") >>
    params: ws!(separated_list!(tag!(","), nom::alpha)) >>
    tag!(")") >>
    tag!(":") >>
    opt!(nom::newline) >>
    description: opt!(doc_string) >>
    (
        Item {
            node: ItemKind::Function{
                name: name.to_string(),
                description: description,
                parameters: params.iter().map(|x| std::str::from_utf8(x).unwrap().to_string()).collect::<Vec<_>>()
            }
        }
    )
));

named!(doc_string<String>, 
    do_parse!(
        doc_string: map_res!(ws!(delimited!(tag!("\"\"\""), is_not!("\"\"\""), tag!("\"\"\""))), std::str::from_utf8) >>
        (doc_string.to_string())
    )
);

pub fn parse(source: String) {
    let result = item(source.trim().as_bytes());

    println!("The result is {:?}", result);
}
