#![feature(option_result_contains)]

pub mod validator;
mod extension;

#[cfg(test)]
mod tests {
    use crate::validator::Validator;

    #[test]
    fn it_works() {
        let validator = Validator::new();
        assert!(validator.is_valid("txt", "test.txt"))
    }
}
