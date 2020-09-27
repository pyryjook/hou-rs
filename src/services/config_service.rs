use std::fmt;
use snafu::{Snafu, ResultExt};

use crate::services::toml_service::{TomlFileService, TomlFileServiceTrait, TomlFileError};
use crate::domain::objects::Config;
use crate::services::file_path_service::{FilePathService, FilePathError};

// one single error module instead of many!!! so move this there
#[derive(Debug, Snafu)]
pub enum ConfigFileError {
    #[snafu(display("Could not open config from {}: {}", config_path, source))]
    ConfigPath {
        config_path: String,
        source: FilePathError,
    },
    #[snafu(display("Could not read config to {}: {}", config_path, source))]
    ReadConfig {
        config_path: String,
        source: TomlFileError,
    }
}


pub struct ConfigFileService {
    toml_service: TomlFileService,
    config_path: String
}

impl ConfigFileService {
    fn new(toml_service: TomlFileService, config_path: String) -> ConfigFileService {
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

        return match get_config {
            Ok(config) => config,
            Err(_) => default,
            _ => default
        }
    }
}

#[cfg(test)]
mod test {

}