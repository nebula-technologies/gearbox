use crate::rails::ext::syn::if_then::RailsIfExt;
use crate::rails::ext::syn::map_into::RailsMapErrInto;
use crate::rails::tracing::common::RailsLog;
use alloc::string::String;
use libc::FILE;
use spin::{Mutex, MutexGuard};
use std::fs::{create_dir_all, File as StdFile, OpenOptions};
use std::io::{Error, Read, Write};
use std::ops::DerefMut;
use std::path::PathBuf;

pub mod error;
pub mod ext;

pub struct FileWrapper {
    file: Option<StdFile>,
    path: PathBuf,
    read: bool,
    write: bool,
    create: bool,
}

impl FileWrapper {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file: None,
            path,
            read: false,
            write: false,
            create: false,
        }
    }
    pub fn file(
        &mut self,
        read: bool,
        write: bool,
        create: bool,
    ) -> Result<&mut StdFile, error::Error> {
        match (
            self.read == read,
            self.write == write,
            self.create == create,
            self.file.is_some(),
        ) {
            (true, true, true, true) => self
                .file
                .as_mut()
                .ok_or(error::Error::ShouldHaveBeenInfallible),
            _ => self.reopen(read, write, create),
        }
    }

    pub fn reopen(
        &mut self,
        read: bool,
        write: bool,
        create: bool,
    ) -> Result<&mut StdFile, error::Error> {
        if self.file.is_some() {
            self.file = None;
        }

        if create {
            if let Some(parent) = self.path.parent() {
                create_dir_all(parent)?;
            }
        }

        OpenOptions::new()
            .read(read)
            .write(write)
            .create(create)
            .open(&self.path)
            .map(|t| {
                self.file = Some(t);
                self.read = read;
                self.write = write;
                self.create = create;
                self.file.as_mut().unwrap()
            })
            .map_err_into()
    }
}

pub struct File {
    file_access_data: FileAccessData,
    inner: Mutex<FileWrapper>,
}

impl File {
    pub fn new(path: PathBuf, _content_type: Option<ContentType>) -> Self {
        Self {
            file_access_data: FileAccessData { path: path.clone() },
            inner: Mutex::new(FileWrapper::new(path)),
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self::new(path, None)
    }

    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.file_access_data.path
    }

    pub fn contents(&self) -> Result<Vec<u8>, error::Error> {
        self.wrapper()
            .file(true, true, true)
            .and_then(|mut t| Self::contents_map(t).map_err_into())
            .log(crate::error!(Err))
    }

    pub fn contents_string(&self) -> Result<String, error::Error> {
        self.wrapper()
            .file(true, true, true)
            .and_then(|t| Self::contents_string_map(t).map_err_into())
            .log(crate::error!(Err))
    }

    pub fn write_to_file(
        &mut self,
        content: &[u8],
    ) -> Result<(), crate::storage::io::file::error::Error> {
        self.wrapper()
            .file(false, true, true)
            .and_then(|t| t.write_all(content).map_err(error::Error::from))
    }

    pub fn write_str_to_file(&mut self, content: &str) -> Result<(), error::Error> {
        self.wrapper()
            .file(false, true, true)
            .and_then(|t| t.write_all(content.as_bytes()).map_err(error::Error::from))
    }

    pub fn exists(&self) -> bool {
        self.file_access_data.path.exists()
    }

    fn wrapper(&self) -> MutexGuard<'_, FileWrapper> {
        self.inner.lock()
    }
    fn contents_map(t: &mut StdFile) -> Result<Vec<u8>, std::io::Error> {
        let mut contents = Vec::new();
        t.read_to_end(&mut contents).map(|_| contents)
    }
    fn contents_string_map(t: &mut StdFile) -> Result<String, std::io::Error> {
        let mut contents = String::new();
        t.read_to_string(&mut contents).map(|_| contents)
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new(PathBuf::new(), None)
    }
}

pub struct FileAccessData {
    path: PathBuf,
    //content_type: Option<ContentType>,
}

pub enum ContentType {
    Json,
}

#[cfg(test)]
mod tests {
    use super::File;
    use std::fs;
    use std::fs::remove_file;
    use std::io::{Read, Write};
    use std::path::PathBuf;

    #[test]
    fn test_file_create_and_read_manual() {
        let path = PathBuf::from("/tmp/test_file_manual.json");
        let content = r#"{"key": "value"}"#.to_string();

        // Write to file manually
        {
            let mut file = std::fs::File::create(&path).expect("Failed to create file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to file");
        }

        // Read from file manually
        {
            let mut file = std::fs::File::open(&path).expect("Failed to open file");
            let mut read_content = String::new();
            file.read_to_string(&mut read_content)
                .expect("Failed to read from file");
            assert_eq!(read_content, content);
        }

        // Clean up
        fs::remove_file(path).expect("Failed to delete test file");
    }

    #[test]
    fn test_file_create_and_read() {
        remove_file("/tmp/rust-test/storage/io/file/ext/tests/test_file-1.json").ok();
        let path = PathBuf::from("/tmp/rust-test/storage/io/file/ext/tests/test_file-1.json");
        let mut file = File::new(path.clone(), None);
        let content = r#"{"key": "value"}"#.to_string();

        file.write_str_to_file(&content)
            .expect("Failed to write to file");
        let read_content = file.contents_string().expect("Failed to read from file");
        assert_eq!(read_content, content);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_file_manual_create_and_simplefile_read() {
        let path = PathBuf::from("/tmp/test_file_manual-1.json");
        let content = r#"{"key": "value"}"#.to_string();

        // Write to file manually
        {
            let mut file = std::fs::File::create(&path).expect("Failed to create file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to file");
        }
        let mut file = File::new(path.clone(), None);
        let read_content = file.contents_string().expect("Failed to read from file");
        assert_eq!(read_content, content);

        fs::remove_file(path).unwrap();
    }
}
