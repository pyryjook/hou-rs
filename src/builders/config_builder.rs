use crate::domain::objects::Config;
use crate::services::toml_service::TomlFileService;


pub struct ConfigBuilder {
    config_path: Option<String>,
    toml_service: Option<TomlFileService>
}


impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder{
            toml_service: None
        }
    }

    pub from(&mut self, config_path: String) &mut Self -> {

    }

    fn build(self) -> Config {

    }
}