use std::fmt::Formatter;

// #[derive(Debug)]
// #[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
// #[cfg_attr(feature = "napi", napi(string_enum))]
#[derive(uniffi::Error, Debug)]
pub enum GDError {
    NumberTooLarge,
    NumberTooSmall,
    InitError,
}

impl std::fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GDError::NumberTooLarge => write!(f, "Number too large"),
            GDError::NumberTooSmall => write!(f, "Number too small"),
            GDError::InitError => write!(f, "Init error"),
        }
    }
}
