use crate::uniffi_rs::error::UniffiError;

mod error;
mod init;

#[derive(uniffi::Object)]
pub struct MathManager;

#[uniffi::export(callback_interface)]
pub trait CallbackTrait {
    fn call(&self, message: String);
}

#[uniffi::export]
impl MathManager {
    #[uniffi::constructor]
    fn new() -> Self {
        MathManager
    }

    fn multiply(&self, left: i32, right: i32) -> i32 {
        left * right
    }

    fn test_callback(&self, callback: Box<dyn CallbackTrait>) {
        callback.call("hello rust".to_string());
    }
}

/// 判断一个数是否为奇数
/// 但是这个数只能在 0 到 100 之间
// #[cfg_attr(feature = "uniffi", uniffi::export)]
// #[cfg_attr(feature = "napi", napi)]
#[uniffi::export]
pub async fn is_odd(num: i32) -> Result<bool, UniffiError> {
    if num < 0 {
        return Err(UniffiError::NumberTooSmall);
    }

    if num > 100 {
        return Err(UniffiError::NumberTooLarge);
    }

    Ok(num % 2 == 1)
}
