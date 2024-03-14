use crate::rails::ext::if_then::RailsIfExt;
use crate::rails::ext::map_into::RailsMapErrInto;
use crate::rails::tracing::common::{RailsLog, RailsLogState};
use std::fs::File as StdFile;
use std::io::{Read, Write};
use std::path::PathBuf;

pub mod error;
pub mod ext;

pub struct File {
    file_access_data: FileAccessData,
    inner: Option<StdFile>,
}

impl File {
    pub fn new(path: PathBuf, _content_type: Option<ContentType>) -> Self {
        Self {
            file_access_data: FileAccessData { path },
            inner: None,
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self::new(path, None)
    }

    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.file_access_data.path
    }

    pub fn get_contents_mut(&mut self) -> Result<String, crate::storage::io::file::error::Error> {
        self.create_if_not_exists()
            .and_then(|t| Self::contents_map(t).map_err_into())
            .log(Err.error())
    }

    pub fn get_contents(&self) -> Result<String, crate::storage::io::file::error::Error> {
        self.get_if_exists()
            .and_then(|t| Self::contents_map(&t).map_err_into())
            .log(Err.error())
    }

    pub fn write_to_file(
        &mut self,
        content: String,
    ) -> Result<(), crate::storage::io::file::error::Error> {
        self.create_if_not_exists().and_then(|t| {
            t.write_all(content.as_bytes())
                .map_err(crate::storage::io::file::error::Error::from)
        })
    }

    pub fn exists(&self) -> bool {
        self.file_access_data.path.exists()
    }

    fn create_if_not_exists(
        &mut self,
    ) -> Result<&mut std::fs::File, crate::storage::io::file::error::Error> {
        match &self.inner {
            None => { self.exists() }
                .if_then(|t| *t)
                .ok_or(error::Error::FileDoesNotExist)
                .and_then(|_| std::fs::File::open(&self.file_access_data.path).map_err_into())
                .or_else(|_e| std::fs::File::create(&self.file_access_data.path).map_err_into())
                .map(|t| {
                    self.inner = Some(t);
                    self.inner.as_mut().unwrap()
                }),
            Some(_) => self
                .inner
                .as_mut()
                .ok_or(crate::storage::io::file::error::Error::FileDoesNotExist),
        }
    }
    fn get_if_exists(&self) -> Result<std::fs::File, crate::storage::io::file::error::Error> {
        match &self.inner {
            None => { self.exists() }
                .if_then(|t| *t)
                .ok_or(error::Error::FileDoesNotExist)
                .and_then(|_| std::fs::File::open(&self.file_access_data.path).map_err_into()),
            Some(t) => t.try_clone().map_err_into(),
        }
    }

    fn contents_map(t: &StdFile) -> Result<String, std::io::Error> {
        let mut contents = String::new();
        t.try_clone()
            .and_then(|mut t| t.read_to_string(&mut contents).map(|_| contents))
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
