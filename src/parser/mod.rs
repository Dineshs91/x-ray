use std;

use nom;
use structures::{Module, Class, Function};

named!(item_module, alt!(
    item_class
    |
    item_fn
    |
    doc_string
));

named!(item_class<Class>, do_parse!(
    tag!("class") >>
    many1!(nom::space) >>
    name: map_res!(nom::alpha, std::str::from_utf8) >>
    description: opt!(doc_string) >>
    methods: many0!(item_fn) >>
    (Class {
        name: name.to_string(),
        description: description,
        methods: methods
    })
));

named!(item_fn<Function>, do_parse!(
    tag!("def") >>
    many1!(nom::space) >>
    name: map_res!(nom::alpha, std::str::from_utf8) >>
    tag!("(") >>
    params: ws!(separated_list!(tag!(","), nom::alpha)) >>
    tag!(")") >>
    tag!(":") >>
    opt!(nom::newline) >>
    description: opt!(doc_string) >>
    (Function {
        name: name.to_string(), 
        description: description,
        parameters: params.iter().map(|x| std::str::from_utf8(x).unwrap().to_string()).collect::<Vec<_>>()
    })
));

named!(doc_string<String>, 
    do_parse!(
        doc_string: map_res!(ws!(delimited!(tag!("\"\"\""), is_not!("\"\"\""), tag!("\"\"\""))), std::str::from_utf8) >>
        (doc_string.to_string())
    )
);

pub fn parse(source: String) {
    // parse the source and return the package struct.
}