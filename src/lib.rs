#![feature(option_result_contains)]

pub mod validator;
mod extension;

#[cfg(test)]
mod tests {
    use crate::validator::{FileExtensionValidator, Validator};

    #[test]
    fn it_works() {
        let validator = FileExtensionValidator::new();
        let valid = validator.is_valid("txt", "atest.txt");
        assert!(!valid);
        let valid = validator.is_valid("txt", "test.txt");
        assert!(valid);
    }
}
