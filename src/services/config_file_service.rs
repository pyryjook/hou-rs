use crate::services::toml_service::{TomlFileService, TomlFileServiceTrait};
use crate::domain::objects::Config;
use crate::services::file_path_service::FilePathService;


pub struct ConfigFileService {
    toml_service: TomlFileService,
    config_path: String
}

impl ConfigFileService {
    pub fn new(config_path: String, toml_service: TomlFileService) -> ConfigFileService {
        ConfigFileService {
            toml_service,
            config_path
        }
    }

    fn get_empty_config(&self) -> Config {
        return Config {
            lex_office_api_key: None
        }
    }

    pub fn read_config(self) -> Config {
        let default = self.get_empty_config();
        let get_config= move || {
            let path = FilePathService::absolute_path(&self.config_path)?;
            return self.toml_service.read_from_file::<Config>(&path);
        };

        return match get_config() {
            Ok(config) => config,
            Err(e) => {
                println!("No config found, moving forward with default config: {}", e);
                return default
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::services::file_service::{FileService, FileServiceTrait};

    #[test]
    fn read_config_actual() {
        let expected = Config{lex_office_api_key: Some("apikey".to_string())};
        let config_file_service = ConfigFileService::new(
            "test_helpers/mock_config.toml".to_string(),
            TomlFileService::new(FileService::new())
        );

        let res = config_file_service.read_config();
        assert_eq!(res, expected)
    }

    #[test]
    fn read_config_default() {
        let expected = Config{lex_office_api_key: None};
        let config_file_service = ConfigFileService::new(
            "/mock_config.toml".to_string(),
            TomlFileService::new(FileService::new())
        );

        let res = config_file_service.read_config();
        assert_eq!(res, expected)
    }
}