//! This module is responsible to manage paths inside Rome.
//! It is a small wrapper around [path::PathBuf] but it is also able to
//! give additional information around the the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use std::io::Read;
use std::{fs::File, io::Write, ops::Deref, path::PathBuf};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct RomePath {
    file: PathBuf,
}

impl Deref for RomePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl RomePath {
    pub fn new(path_to_file: &str) -> Self {
        Self {
            file: PathBuf::from(path_to_file),
        }
    }

    // TODO: handle error with diagnostic?
    /// Opens a file and returns a [File] in write mode
    pub fn open(&self) -> File {
        File::open(&self.file).expect("cannot open the file to format")
    }

    /// Accepts a file opened in read mode and saves into it
    pub fn save(&mut self, content: &str) -> Result<(), std::io::Error> {
        let mut file_to_write = File::create(&self.file).unwrap();
        // TODO: handle error with diagnostic
        file_to_write.write_all(content.as_bytes())
    }

    /// Returns the contents of a file, if it exists
    pub fn get_buffer_from_file(&mut self) -> String {
        let mut file = self.open();
        let mut buffer = String::new();
        // we assume we have permissions
        file.read_to_string(&mut buffer)
            .expect("cannot read the file to format");

        buffer
    }
}
