extern crate x_ray;

mod util;

use std::env;

fn get_current_directory() -> String {
    let cwd = env::current_dir().unwrap();
    cwd.to_str().unwrap().to_string()
}

#[test]
fn test_src_parse_gen() {
    let conf_file_gen = "tests/test_gen.toml".to_string();
    let conf_file_parse = "tests/test_parse.toml".to_string();

    // Test codegen.
    let test_gen_toml_content = util::read_file(&conf_file_gen);
    x_ray::gen(get_current_directory() + "/tests/test_py_project", conf_file_gen);

    // Test parse.
    let parse_dir = "tests/test_py_project".to_string();
    x_ray::parse(&parse_dir, &conf_file_parse);
    let test_parse_toml_content = util::read_file(&conf_file_parse);

    assert_eq!(test_gen_toml_content, test_parse_toml_content);
}
