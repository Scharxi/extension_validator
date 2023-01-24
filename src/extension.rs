use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// A trait to get the extension of an file
pub trait Extension<T> where T: AsRef<OsStr> {
    type Error;
    /// Function that returns the extension of an file 
    fn get_extension(file: T) -> Result<String, Self::Error>;
}

impl<T> Extension<T> for T where T: AsRef<OsStr> + Into<String> {
    type Error = Option<OsString>;
    fn get_extension(file: T) -> Result<String, Option<OsString>> {
        let path: &String = &file.into();
        let path = Path::new(path);
        if !path.exists() {
            return Err(None);
        }
        let extension = path
            .extension();
        match extension {
            Some(ext) => Ok(ext.to_os_string().into_string().unwrap()),
            None => Err(None)
        }
    }
}