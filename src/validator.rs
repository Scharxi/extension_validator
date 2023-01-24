use std::ffi::OsStr;
use crate::extension::Extension;

#[derive(Default)]
pub struct FileExtensionValidator {}

pub trait Validator {
    /// Checks whether the extension of the file matches the `required` extension
    /// # Example
    /// ```
    ///use extension_validator::validator::Validator;
    ///let validator = Validator::new();
    ///assert!(validator.is_valid("txt", "test.txt"))
    /// ```
    /// # Returns
    /// `false` if the file can't be found or the extension of the file doesn't match the required one
    /// `true` in all other cases
    fn is_valid<S, T>(&self, required: S, to_check: T) -> bool
        where S: Into<String>,
              T: AsRef<OsStr> + Into<String>;
}

impl FileExtensionValidator {
    /// Creates a new validator
    pub fn new() -> Self { Self::default() }
}

impl Validator for FileExtensionValidator {
    fn is_valid<S, T>(&self, required: S, to_check: T) -> bool where S: Into<String>, T: AsRef<OsStr> + Into<String> {
        T::get_extension(to_check).contains(&required.into())
    }
}