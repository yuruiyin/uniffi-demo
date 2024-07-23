// uniffi 和 napi 公共的接口
use lazy_static::lazy_static;
#[cfg(feature = "napi")]
use napi_derive_ohos::napi;
use tokio::sync::RwLock;

lazy_static! {
    /// 是否已经初始化过了，防止多次初始化
    pub static ref IS_INITED: RwLock<bool> = RwLock::new(false);
}

/// 原生平台的相关配置
#[derive(Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct AppConfig {
    // app 版本号, 如 1.0.0
    pub version: String,

    // 环境，如 prod, stage, fat, dev
    pub env: String,

    // 用户id
    pub user_id: String,
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "napi", napi)]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "napi", napi)]
pub async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "napi", napi)]
pub async fn async_minus(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "napi", napi)]
pub fn add_input(input: Input) -> Output {
    Output {
        result: add(input.left, input.right),
    }
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct Input {
    pub left: i32,
    pub right: i32,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[cfg_attr(feature = "napi", napi(object))]
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
