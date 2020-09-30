use std::env;
use snafu::ResultExt;

use crate::constants::{HOME_ENV_KEY};
use crate::domain::errors::toml_file::EnvVariableError;
use crate::domain::errors::toml_file::FileError;

pub struct FilePathService;

impl FilePathService {
    pub fn absolute_path(path: &String) -> Result<String, FileError> {
        if !path.starts_with("~") {
            return Ok(String::from(path))
        }
        let home_path = env::var(HOME_ENV_KEY).context(EnvVariableError {variable: HOME_ENV_KEY.to_string()})?;

        return Ok(format!("{}{}", home_path, path))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_config_file_path_with_home() {
        let path= "~/.hou-rs/config".to_string();

        let res = FilePathService::absolute_path(&path).unwrap();

        let expected = ".hou-rs/config";

        assert_eq!(res.contains(expected), true);
        assert_eq!(res.len() > expected.len(), true);
    }

    #[test]
    fn test_get_config_file_path_with_absolute() {
        let path= "path/to/.hou-rs/config".to_string();

        let res = FilePathService::absolute_path(&path).unwrap();

        assert_eq!(res == path, true);
    }

}