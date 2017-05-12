extern crate x_ray;

mod util;

use std::env;
use std::path::Path;

fn get_current_directory() -> String {
    let cwd = env::current_dir().unwrap();
    cwd.to_str().unwrap().to_string()
}

#[test]
fn test_src_parse_gen() {
    let conf_file_gen = "tests/test_input/conf/test_gen.toml";
    let conf_file_parse = "tests/test_output/conf/test_parse.toml";

    // Test codegen.
    let test_gen_toml_content = util::read_file(conf_file_gen);
    let parse_dir = "tests/test_output/src/test_py_project";
    x_ray::gen(get_current_directory() + "/" + parse_dir, conf_file_gen);

    // Test parse.
    x_ray::parse(parse_dir, conf_file_parse);
    let test_parse_toml_content = util::read_file(conf_file_parse);

    assert_eq!(test_gen_toml_content, test_parse_toml_content);

    // clean test output directory
    let output_src_path = Path::new("tests/test_output/src");
    let output_config_path = Path::new("tests/test_output/conf");

    util::clean_dir(output_src_path);
    util::clean_dir(output_config_path);
}
