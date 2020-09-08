
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub struct FileService {
    file_name: String
}

impl FileService {
    fn read_file_to_string(&self) -> String {
        let path = Path::new(&self.file_name);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut file_content = String::new();

        if let Err(why) = file.read_to_string(&mut file_content) {
            panic!("couldn't read: {}", why)
        }

        return file_content;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        let fs = FileService { file_name: String::from("test_helpers/file.txt") };

        let res = fs.read_file_to_string();

        assert_eq!(&res, "used for testing")
    }
}