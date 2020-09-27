use crate::domain::objects::Config;
use crate::services::toml_service::{TomlFileService, TomlFileServiceTrait};
use crate::services::file_service::{FileService, FileServiceTrait};
use crate::services::file_path_service::FilePathService;


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

    pub fn from(&mut self, config_path: String) -> &mut Self {
        self.config_path = Some(config_path);

        self
    }

    pub fn using_toml(&mut self) -> &mut Self {
        let file_service = FileService::new();
        self.toml_service = Some(TomlFileService::new(file_service));

        self
    }

    fn build(self) -> Option<Config> {
        return match FilePathService::absolute_path(&self.config_path?) {
            Err(_) => None,
            Ok(path) => self.toml_service?.read_from_file::<Config>(&path).ok()
        };

    }
}