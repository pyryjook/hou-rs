use snafu::OptionExt;

use crate::domain::objects::Config;
use crate::services::toml_service::{TomlFileService, TomlFileServiceTrait};
use crate::services::file_service::{FileService, FileServiceTrait};
use crate::services::config_file_service::ConfigFileService;
use crate::domain::errors::config_builder::{ConfigBuilderError, NoneError};


pub struct ConfigBuilder {
    config_path: Option<String>,
    toml_service: Option<TomlFileService>
}


impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder{
            config_path: None,
            toml_service: None
        }
    }

    pub fn from(self, config_path: String) -> ConfigBuilder {
        ConfigBuilder {
            config_path: Some(config_path),
            toml_service: self.toml_service
        }
    }

    pub fn using_toml(self) -> ConfigBuilder {
        let file_service = FileService::new();

        ConfigBuilder {
            config_path: self.config_path,
            toml_service: Some(TomlFileService::new(file_service))
        }
    }

    pub fn build(self) -> Result<Config, ConfigBuilderError>  {
        let config_file_service = ConfigFileService::new(
            self.config_path.context(NoneError)?,
            self.toml_service.context(NoneError)?
        );

        return Ok(config_file_service.read_config());
    }
}

mod test {
    use super::*;

    #[test]
    fn test_builder_found_config() {
        let expected = Config{lex_office_api_key: Some("apikey".to_string())};

        let config = ConfigBuilder::new()
            .using_toml()
            .from("test_helpers/mock_config.toml".to_owned())
            .build().unwrap();

        assert_eq!(config, expected)
    }

    #[test]
    fn test_builder_default_config() {
        let expected = Config{lex_office_api_key: None};

        let config = ConfigBuilder::new()
            .using_toml()
            .from("/mock_config.toml".to_owned())
            .build().unwrap();

        assert_eq!(config, expected)
    }


    #[test]
    fn test_builder_error() {
        let config = ConfigBuilder::new()
            .from("/mock_config.toml".to_owned())
            .build();

        assert_eq!(config.is_err(), true)
    }
}