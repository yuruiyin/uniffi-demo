#[cfg(feature = "napi")]
use napi_derive_ohos::napi;

#[cfg_attr(feature = "napi", napi)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug)]
pub enum AppLogLevel {
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
}

/// 日志输入
#[cfg_attr(feature = "napi", napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug)]
pub struct AppLog {
    pub level: AppLogLevel,
    pub tag: String,
    pub message: String,
}