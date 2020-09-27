use std::{env, fmt};
use snafu::{Snafu, ResultExt};

use crate::constants::{HOME_ENV_KEY};

#[derive(Debug, Snafu)]
pub enum FilePathError {
    #[snafu(display("Could not find env variable: {}", source))]
    EnvError{
        source: std::env::VarError,
    },
}

pub struct FilePathService;

impl FilePathService {
    pub fn absolute_path(path: &String) -> Result<String, FilePathError> {
        if !path.starts_with("~") {
            return Ok(String::from(path))
        }
        let home_path = env::var(HOME_ENV_KEY).context(EnvError)?;

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

        assert_eq!(res.contains(".hou-rs/config"), true);
        assert_eq!(res.contains("Users"), true);
    }

    #[test]
    fn test_get_config_file_path_with_absolute() {
        let path= "path/to/.hou-rs/config".to_string();

        let res = FilePathService::absolute_path(&path).unwrap();

        assert_eq!(res == path, true);
    }

}