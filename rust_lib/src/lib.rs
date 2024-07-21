use crate::error::GDError;

uniffi::setup_scaffolding!();

mod error;
mod init;

#[derive(uniffi::Object)]
pub struct MathManager;

// #[derive(uniffi::Record)]
// pub struct Callback;

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
#[uniffi::export]
pub async fn is_odd(num: i32) -> Result<bool, GDError> {
    if num < 0 {
        return Err(GDError::NumberTooSmall);
    }

    if num > 100 {
        return Err(GDError::NumberTooLarge);
    }

    Ok(num % 2 == 1)
}

#[uniffi::export]
pub async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

#[uniffi::export]
pub async fn async_minus(left: i32, right: i32) -> i32 {
    left - right
}

#[uniffi::export]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[uniffi::export]
pub fn add_input(input: Input) -> Output {
    Output {
        result: add(input.left, input.right),
    }
}

#[derive(uniffi::Record)]
pub struct Input {
    pub left: i32,
    pub right: i32,
}

#[derive(uniffi::Record)]
pub struct Output {
    pub result: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
