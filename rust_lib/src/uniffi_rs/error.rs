use std::fmt::Formatter;

// #[derive(Debug)]
// #[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
// #[cfg_attr(feature = "napi", napi(string_enum))]
#[derive(uniffi::Error, Debug)]
pub enum UniffiError {
    NumberTooLarge,
    NumberTooSmall,
    InitError(String),
}

impl std::fmt::Display for UniffiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UniffiError::NumberTooLarge => write!(f, "Number too large"),
            UniffiError::NumberTooSmall => write!(f, "Number too small"),
            UniffiError::InitError(message) => write!(f, "Init error: {}", message),
        }
    }
}
