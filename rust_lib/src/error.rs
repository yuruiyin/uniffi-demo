use std::fmt::Formatter;

#[derive(uniffi::Error, Debug)]
pub enum GDError {
    NumberTooLarge,
    NumberTooSmall,
    InitError(String),
}

impl std::fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GDError::NumberTooLarge => write!(f, "Number too large"),
            GDError::NumberTooSmall => write!(f, "Number too small"),
            GDError::InitError(error_msg) => write!(f, "Init error: ${error_msg}"),
        }
    }
}
