extern crate x_ray;

mod util;

use std::path::Path;

use x_ray::cli::ConfType;


#[test]
fn test_src_parse_gen_toml() {
    util::create_test_dirs().unwrap();

    let conf_file_gen = "tests/test_input/conf/test_gen.toml";
    let conf_file_parse = "tests/test_output/conf/test_parse.toml";

    // Test codegen.
    let test_gen_toml_content = util::read_file(conf_file_gen);
    let parse_dir = "tests/test_output/src/test_py_project";
    x_ray::gen(util::get_current_directory() + "/" + parse_dir, conf_file_gen, ConfType::Toml);

    // Test parse.
    x_ray::util::write_to_config(&conf_file_parse, x_ray::parse(parse_dir, ConfType::Toml));
    let test_parse_toml_content = util::read_file(conf_file_parse);

    assert_eq!(test_gen_toml_content, test_parse_toml_content);

    // clean test output directory
    let output_src_path = Path::new("tests/test_output/src");
    let output_config_path = Path::new("tests/test_output/conf");

    util::clean_dir(output_src_path);
    util::clean_dir(output_config_path);
}

#[test]
fn test_src_parse_gen_yaml() {
    util::create_test_dirs().unwrap();

    let conf_file_gen = "tests/test_input/conf/test_gen.yaml";
    let conf_file_parse = "tests/test_output/conf/test_parse.yaml";

    // Test codegen.
    let test_gen_yaml_content = util::read_file(conf_file_gen);
    let parse_dir = "tests/test_output/src/test_py_project";
    x_ray::gen(util::get_current_directory() + "/" + parse_dir, conf_file_gen, ConfType::Yaml);

    // Test parse.
    x_ray::util::write_to_config(&conf_file_parse, x_ray::parse(parse_dir, ConfType::Yaml));
    let test_parse_yaml_content = util::read_file(conf_file_parse);

    assert_eq!(test_gen_yaml_content, test_parse_yaml_content);

    // clean test output directory
    let output_src_path = Path::new("tests/test_output/src");
    let output_config_path = Path::new("tests/test_output/conf");

    util::clean_dir(output_src_path);
    util::clean_dir(output_config_path);
}

#[test]
fn test_src_main() {
    // Create test directories
    util::create_test_dirs().unwrap();

    let src_input = "tests/test_input/src";

    let actual_output_str = x_ray::parse(src_input, ConfType::Toml);
    let exepected_output_str = 
r#"[root]
name = "tests/test_input/src"
packages = []

[[root.modules]]
classes = []
description = "This is the main module."
name = "main"

[[root.modules.functions]]
description = "Main function"
name = "main"
parameters = []
"#;
    assert_eq!(actual_output_str, exepected_output_str);

    let output_src_path = Path::new("tests/test_output/src");
    let output_config_path = Path::new("tests/test_output/conf");

    util::clean_dir(output_src_path);
    util::clean_dir(output_config_path);
}