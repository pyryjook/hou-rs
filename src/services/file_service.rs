
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

pub struct FileReadError;

impl fmt::Display for FileReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error occurred when reading the file")
    }
}

impl fmt::Debug for FileReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub struct FileWriteError;


impl fmt::Display for FileWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error occurred when writing the file")
    }
}

impl fmt::Debug for FileWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}


pub trait FileServiceTrait {
    fn new() -> Self;
    fn read_file_to_string(&self, file_name: String) -> Result<String, FileReadError>;
    fn write_file_from_string(&self, file_name: String, content: String) -> Result<(), FileWriteError>;
}

pub struct FileService;

impl FileServiceTrait for FileService {
    fn new() -> FileService {
        FileService
    }
    fn read_file_to_string(&self, file_name: String) -> Result<String, FileReadError> {
        let path = Path::new(&file_name);

        let mut file = match File::open(&path) {
            Err(_) => return Err(FileReadError),
            Ok(file) => file,
        };

        let mut file_content = String::new();

        if let Err(_) = file.read_to_string(&mut file_content) {
            return Err(FileReadError)
        }

        return Ok(file_content);
    }

    fn write_file_from_string(&self, file_name: String, content: String) -> Result<(), FileWriteError> {
        let path = Path::new(&file_name);

        let mut file = match File::create(&path) {
            Err(_) => return Err(FileWriteError),
            Ok(file) => file,
        };

        match file.write_all(content.as_bytes()) {
            Err(_) => return Err(FileWriteError),
            Ok(_) => return Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        let fs = FileService::new();

        let res = fs.read_file_to_string(String::from("test_helpers/file.txt"));

        assert_eq!(res.is_ok(), true)
    }
    #[test]
    fn test_read_file_to_string_error() {
        let fs = FileService::new();

        let res = fs.read_file_to_string( String::from("non_existent/file.txt"));

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_write_file_to_string() {
        let fs = FileService::new();

        let res = fs.write_file_from_string(String::from("test_helpers/write_file.txt"), String::from("this is a test"));

        assert_eq!(res.is_ok(), true)
    }

    #[test]
    fn test_write_file_to_string_error() {
        let fs = FileService::new();

        let res = fs.write_file_from_string(String::from("/write_file.txt"), String::from("this is a test"));

        assert_eq!(res.is_err(), true)
    }
}