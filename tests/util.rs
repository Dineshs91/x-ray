use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::{self, File};

pub fn read_file(filename: &str) -> String {
    let file = File::open(filename);

    let mut file_content = String::new();

    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("The following error occurred {:?}", error),
    };

    match file.read_to_string(&mut file_content) {
        Ok(_) => {},
        Err(error) => panic!("There was an error {:?} reading the file {}", error, filename),
    }

    // return the file content.
    file_content
}

pub fn get_current_directory() -> String {
    let cwd = env::current_dir().unwrap();
    cwd.to_str().unwrap().to_string()
}

/// Create test directories
/// tests
///   |--- test_input/conf
///   |--- test_intput/src
///   |--- test_output/conf
///   |--- test_output/src
pub fn create_test_dirs() -> ::std::io::Result<()> {
    fs::create_dir_all("tests/test_input/src")?;
    fs::create_dir_all("tests/test_input/conf")?;
    fs::create_dir_all("tests/test_output/src")?;
    fs::create_dir_all("tests/test_output/conf")?;
    Ok(())
}

/// Remove all the files and nested directories in a directory.
pub fn clean_dir(dir: &Path) -> bool {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let dir_entry = entry.unwrap();
            let path = dir_entry.path();

            if !path.is_dir() {
                match fs::remove_file(path) {
                    Ok(_) => {},
                    Err(_) => panic!("Error removing file")
                }
            } else {
                clean_dir(&path);

                // Remove directory after all the files have been removed.
                match fs::remove_dir(&path) {
                    Ok(_) => {},
                    Err(e) => panic!("Failed to remove directory {} {:?}", e, path)
                }
            }
        }
        return true;
    }

    false
}
