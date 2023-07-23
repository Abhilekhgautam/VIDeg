use std::fs;
pub struct Document<'a> {
    file_name: &'a str,
    content: String,
}

impl<'a> Document<'a> {
    pub fn new(file_path: &'a str) -> Self {
        let contents = fs::read_to_string(file_path).unwrap();
        Self {
            // todo: stripe the path to only contain the file name
            file_name: file_path,
            content: contents,
        }
    }
    pub fn get_contents(&self) -> &str {
        self.content.as_str()
    }
    pub fn get_file_name(&self) -> &str {
        self.file_name
    }
}
