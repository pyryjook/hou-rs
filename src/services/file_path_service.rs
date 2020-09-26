use std::{env, fmt};
use crate::constants::{CONFIG_FILE_PATH, HOME_ENV_KEY};

pub struct FilePathError;

impl fmt::Display for FilePathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error occurred when reading the file")
    }
}

impl fmt::Debug for FilePathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub struct FilePathService;

impl FilePathService {
    pub fn to_absolute(path: String) -> Result<String, FilePathError> {
        if !path.starts_with("~") {
            return Ok(path)
        }
        let home_path = env::var(HOME_ENV_KEY)?;

        return Ok(format!("{}{}", home_path, path))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_config_file_path() {

        let res = FilePathService::to_absolute().unwrap();

        assert_eq!(res.contains(CONFIG_FILE_PATH), true);
        assert_eq!(res.len() > CONFIG_FILE_PATH.len(), true);
    }

}