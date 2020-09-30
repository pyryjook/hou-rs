use serde::de::DeserializeOwned;
use serde::{Serialize};
use snafu::{ResultExt, IntoError};

use crate::services::file_service::{FileService, FileServiceTrait};
use crate::domain::errors::toml_file::{FileError, SerializeToml, DeserializeToml};


pub trait TomlFileServiceTrait {
    fn new(file_service: FileService) -> Self;
    fn read_from_file<T>(&self, file: &String) -> Result<T, FileError> where T: DeserializeOwned;
    fn save_to_file<T>(&self, config: T, file_name: &String) -> Result<(), FileError> where T: Serialize;
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

    fn read_from_file<T>(&self, file_name: &String) -> Result<T, FileError> where T: DeserializeOwned {
        let content = self.file_service.read_file_to_string(file_name)?;

        return toml::from_str(&content).map_err(|e| DeserializeToml { path: file_name }.into_error(e));
    }

    fn save_to_file<T>(&self, config: T, file_name: &String) -> Result<(), FileError> where T: Serialize {
        let toml_str = toml::to_string(&config).context(SerializeToml{path: file_name})?;

        return Ok(self.file_service.write_file_from_string(file_name, toml_str)?);
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
        let res: MockConfig = toml_service.read_from_file(&String::from("test_helpers/read_this.toml")).unwrap();

        assert_eq!(res.title, String::from("TOML Example"));
    }

    #[test]
    fn test_read_from_file_is_ok() {
        let toml_service = TomlFileService::new( FileService::new() );
        let res: Result<MockConfig, FileError>  = toml_service.read_from_file(&String::from("test_helpers/read_this.toml"));

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_read_from_file_is_err() {
        let toml_service = TomlFileService::new( FileService::new() );
        let res: Result<MockConfig, FileError>  = toml_service.read_from_file(&String::from("/read_this.toml"));

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_write_to_file() {
        let config = MockConfig{ title: "foo".to_string() };

        let toml_service = TomlFileService::new( FileService::new() );

        let res = toml_service.save_to_file(config, &String::from("test_helpers/file.toml"));

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_write_to_file_error() {
        let config = MockConfig{ title: "foo".to_string() };

        let toml_service = TomlFileService::new( FileService::new() );

        let res = toml_service.save_to_file(config, &String::from("/file.toml"));

        assert_eq!(res.is_err(), true);
    }
}
