pub trait ValidProvider<T>: Validator + From<T> + Sized {
    fn new(value: T) -> Result<Self, Self::Error> {
        let res = Self::from(value);
        res.check()?;
        Ok(res)
    }
}

pub trait Validator {
    type Error;
    fn check(&self) -> Result<(), Self::Error>;
}

pub trait CratesIoValidator {
    type Error;
    fn check_with_api(&self /* api: &CratesIoApi */) -> Result<(), Self::Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum ValidStrError {
    #[error("Provided string is empty: {0}")]
    EmptyString(String),
    #[error("Provided string starts with a digit: {0}")]
    StartsWithDigit(String),
    #[error("Provided string contains invalid characters: {0}")]
    ContainsInvalidChar(String),
}

// how to use try_from with generics
#[derive(Debug, PartialEq, Eq)]
pub struct ValidStr(String);
impl<S: AsRef<str>> From<S> for ValidStr {
    fn from(value: S) -> Self {
        ValidStr(value.as_ref().to_string())
    }
}
impl Validator for ValidStr {
    type Error = ValidStrError;
    fn check(&self) -> Result<(), Self::Error> {
        match self.0.as_str() {
            s if s.is_empty() || s.trim().is_empty() => {
                Err(ValidStrError::EmptyString(s.to_string()))
            }
            s if s.chars().next().unwrap().is_ascii_digit() => {
                Err(ValidStrError::StartsWithDigit(s.to_string()))
            }
            s if s.chars().any(|c| !c.is_ascii_alphanumeric()) => {
                Err(ValidStrError::ContainsInvalidChar(s.to_string()))
            }
            _ => Ok(()),
        }
    }
}
impl<S: AsRef<str>> ValidProvider<S> for ValidStr {}
