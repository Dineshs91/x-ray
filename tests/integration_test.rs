extern crate x_ray;


use std::env;

fn get_current_directory() -> String {
    let cwd = env::current_dir().unwrap();
    cwd.to_str().unwrap().to_string()
}


#[test]
fn test_src_gen() {
    let conf_file = "tests/test_gen.toml".to_string();
    x_ray::gen(get_current_directory() + "/tests/test_py_gen", conf_file);
}

#[test]
fn test_src_parse() {
    let conf_file_gen = "tests/test_gen.toml".to_string();
    x_ray::gen(get_current_directory() + "/tests/test_py_project", conf_file_gen);

    let conf_file = "tests/test_parse.toml".to_string();
    let parse_dir = "tests/test_py_project".to_string();
    x_ray::parse(parse_dir, conf_file)
}
