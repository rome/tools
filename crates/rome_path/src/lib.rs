//! This module is responsible to manage paths inside Rome.
//! It is a small wrapper around [path::PathBuf] but it is also able to
//! give additional information around the the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use std::fs::read_to_string;
use std::io::Read;
use std::{fs::File, io, io::Write, ops::Deref, path::PathBuf};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RomePath {
    file: PathBuf,
    file_id: Option<usize>,
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
            file_id: None,
        }
    }

    /// Builder pattern. It assigns an ID to the file
    pub fn with_id(mut self, file_id: usize) -> Self {
        self.file_id = Some(file_id);
        self
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

    /// Small wrapper for [read_to_string]
    pub fn read_to_string(&self) -> io::Result<String> {
        let path = self.file.as_path();
        read_to_string(path)
    }

    /// Retrieves the ID assigned to the file. It might not exist, that's why it
    /// returns an [Option]
    pub fn file_id(&self) -> Option<usize> {
        self.file_id
    }

    pub fn extension_as_str(&self) -> &str {
        self.extension()
            .expect("Can't read the file")
            .to_str()
            .expect("Can't read the file")
    }
}
