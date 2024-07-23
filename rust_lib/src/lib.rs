uniffi::setup_scaffolding!();

#[cfg(feature = "uniffi")]
mod uniffi_rs;

#[cfg(feature = "napi")]
mod napi_rs;

mod common;
mod log;
