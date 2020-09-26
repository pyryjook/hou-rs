use std::fmt;
use serde::de::DeserializeOwned;
use serde::{Serialize};

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
    fn read_from_file<T>(&self, file: String) -> Result<T, TomlFileError> where T: DeserializeOwned;
    fn save_to_file<T>(&self, config: T, file_name: String) -> Result<(), TomlFileError> where T: Serialize;
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

    fn read_from_file<T>(&self, file_name: String) -> Result<T, TomlFileError> where T: DeserializeOwned {
        let content = match self.file_service.read_file_to_string(file_name) {
            Err(_) => return Err(TomlFileError),
            Ok(c) => c
        };

        let res = match toml::from_str(&content) {
            Err(_) => Err(TomlFileError),
            Ok(toml_content) => Ok(toml_content)
        };

        return res
    }

    fn save_to_file<T>(&self, config: T, file_name: String) -> Result<(), TomlFileError> where T: Serialize {
        let toml_str = match toml::to_string(&config) {
            Err(_) => return Err(TomlFileError),
            Ok(c) => c
        };

        let res= match self.file_service.write_file_from_string(file_name, toml_str) {
            Err(_) => return Err(TomlFileError),
            Ok(_) => Ok(())
        };

        return res
    }
}

#[cfg(test)]
mod test {
    use serde::{Serialize, Deserialize};
    use super::*;

    #[derive(Deserialize, Serialize)]
    struct MockConfig {
        title: String
    }

    #[test]
    fn test_read_from_file() {
        let toml_service = TomlFileService::new( FileService::new() );
        let res: MockConfig = toml_service.read_from_file(String::from("test_helpers/read_this.toml")).unwrap();

        assert_eq!(res.title, String::from("TOML Example"));
    }

    #[test]
    fn test_read_from_file_content() {
        let toml_service = TomlFileService::new( FileService::new() );
        let res: Result<MockConfig, TomlFileError>  = toml_service.read_from_file(String::from("test_helpers/read_this.toml"));

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_write_to_file() {
        let config = MockConfig{ title: "foo".to_string() };

        let toml_service = TomlFileService::new( FileService::new() );

        let res = toml_service.save_to_file(config, String::from("test_helpers/file.toml"));

        assert_eq!(res.is_ok(), true);
    }
}
