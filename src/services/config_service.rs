use crate::services::toml_service::{TomlFileService, TomlFileServiceTrait, TomlFileError};
use crate::domain::objects::Config;
use crate::services::file_path_service::get_config_file_path;


pub struct ConfigFileService {
    toml_service: TomlFileService
}

impl ConfigFileService {
    fn new(toml_service: TomlFileService) -> ConfigFileService {
        ConfigFileService {
            toml_service
        }
    }

    fn get_empty_config(&self) -> Config {
        return Config {
            lex_office_api_key: None
        }
    }

    pub fn read_config(&self) -> Result<Config, TomlFileError> {
        let file = match get_config_file_path() {
            Err(_) => return Err(TomlFileError),
            Ok(c) => c
        };

        let res: Result<Config, TomlFileError>  = match self.toml_service.read_from_file::<Config>(file) {
            Err(_) => Ok(self.get_empty_config()),
            Ok(config) => Ok(config)
        };

        return res;
    }
}

#[cfg(test)]
mod test {

}