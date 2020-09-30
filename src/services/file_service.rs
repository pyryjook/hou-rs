use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use snafu::ResultExt;

use crate::domain::errors::toml_file::FileError;
use crate::domain::errors::toml_file::ReadFile;
use crate::domain::errors::toml_file::WriteFile;



pub trait FileServiceTrait {
    fn new() -> Self;
    fn read_file_to_string(&self, file_name: &String) -> Result<String, FileError>;
    fn write_file_from_string(&self, file_name: &String, content: String) -> Result<(), FileError>;
}

pub struct FileService;

impl FileServiceTrait for FileService {
    fn new() -> FileService {
        FileService
    }
    fn read_file_to_string(&self, file_name: &String) -> Result<String, FileError> {
        let path = Path::new(file_name);

        let mut file = File::open(&path).context(ReadFile {path: file_name})?;

        let mut file_content = String::new();

        file.read_to_string(&mut file_content).context(ReadFile {path: file_name})?;

        return Ok(file_content);
    }

    fn write_file_from_string(&self, file_name: &String, content: String) -> Result<(), FileError> {
        let path = Path::new(file_name);

        let mut file = File::create(&path).context(WriteFile {path: file_name})?;

        return Ok(file.write_all(content.as_bytes()).context(WriteFile {path: file_name})?);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        let fs = FileService::new();

        let res = fs.read_file_to_string(&String::from("test_helpers/file.txt"));

        assert_eq!(res.is_ok(), true)
    }
    #[test]
    fn test_read_file_to_string_error() {
        let fs = FileService::new();

        let res = fs.read_file_to_string( &String::from("non_existent/file.txt"));

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_write_file_to_string() {
        let fs = FileService::new();

        let res = fs.write_file_from_string(&String::from("test_helpers/write_file.txt"), String::from("this is a test"));

        assert_eq!(res.is_ok(), true)
    }

    #[test]
    fn test_write_file_to_string_error() {
        let fs = FileService::new();

        let res = fs.write_file_from_string(&String::from("/write_file.txt"), String::from("this is a test"));

        assert_eq!(res.is_err(), true)
    }
}
