use std::ffi::OsStr;
use crate::extension::Extension;

#[derive(Default)]
pub struct Validator {}

impl Validator {
    pub fn new() -> Self { Self::default() }

    pub fn is_valid<S, T>(&self, required: S, to_check: T) -> bool
        where S: Into<String>,
              T: AsRef<OsStr> + Into<String> {
        T::get_extension(to_check).contains(&required.into())
    }
}