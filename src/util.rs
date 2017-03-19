use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

const FILE_EXTENSION:&'static str = ".py";
const INIT_FILE:&'static str = "__init__.py";

pub fn write_to_file(path: &str, filename: &str, content: &str) {
	// Write the python source to file.
	// Args: file name, the content of the file.
	//
	// filename & content will be &str since we won't be manipulating it.
	let filename = filename.to_string() + FILE_EXTENSION;

	let path = Path::new(&path).join(filename);
	let mut file = match File::create(&path) {
		Err(e) => panic!("Error occurred while trying to create file {}", e),
		Ok(file) => file,
	};

	match file.write_all(content.as_bytes()) {
		Err(e) => println!("Error occurred while trying to write to file {}", e),
		Ok(_) => println!("Successfully written content to a file"),
	}
}

pub fn create_package(package_name: &str) {
	let package_name = package_name.to_string();
	let init_file_path = INIT_FILE;

	fs::create_dir(&package_name);

	let path = Path::new(&package_name).join(init_file_path);
	match File::create(&path) {
		Err(e) => panic!("Error occurred while trying to create file {}", e),
		Ok(file) => file,
	};
}