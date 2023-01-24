use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

pub trait Extension<T> where T: AsRef<OsStr> {
    type Error;
    fn get_extension(file: T) -> Result<String, Self::Error>;
}

impl<T> Extension<T> for T where T: AsRef<OsStr> + Into<String> {
    type Error = Option<OsString>;
    fn get_extension(file: T) -> Result<String, Option<OsString>> {
        let path: &String = &file.into();
        let extension = Path::new(path)
            .extension();
        match extension {
            Some(ext) => Ok(ext.to_os_string().into_string().unwrap()),
            None => Err(None)
        }
    }
}