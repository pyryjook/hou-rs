use std::fmt;

use toml::Value;
use crate::services::file_service::{FileService, FileServiceTrait};

pub struct TomlFileError;

impl fmt::Display for TomlFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error occurred when trying to read reading the TOML file")
    }
}

impl fmt::Debug for TomlFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub trait TomlFileServiceTrait {
    fn new(file_service: FileService) -> Self;
    fn read_from_file(&self, file: String) -> Result<Value, TomlFileError>;
}

pub struct TomlFileService {
    file_service: FileService
}

impl TomlFileServiceTrait for TomlFileService {
    fn new(file_service: FileService) -> TomlFileService {
        TomlFileService {
            file_service
        }
    }
    fn read_from_file(&self, file: String) -> Result<Value, TomlFileError> {
        let content = match &self.file_service.read_file_to_string(file) {
            Err(_) => return Err(TomlFileError),
            Ok(c) => String::from(c)
        };



        let res = match content.parse::<Value>() {
            Err(_) => Err(TomlFileError),
            Ok(toml_content) => Ok(toml_content)
        };

        return res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_from_file() {


        let toml_service = TomlFileService::new( FileService::new() );

        let res: Result<Value, TomlFileError> = toml_service.read_from_file(String::from("test_helpers/read_this.toml"));

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_read_from_file_content() {


        let toml_service = TomlFileService::new( FileService::new() );

        let res: Result<Value, TomlFileError> = toml_service.read_from_file(String::from("test_helpers/read_this.toml"));

        let content = res.unwrap();

        assert_eq!(content["title"].as_str(), Some("TOML Example"));
    }
}
