use lazy_static::lazy_static;
use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::{Function, Promise, Result};
use napi_ohos::threadsafe_function::ThreadsafeFunction;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref TSFN_REF: RwLock<Option<ThreadsafeFunction<VersionInfoRequestModel, Promise<VersionInfoResponseModel>>>> =
        RwLock::new(None);
}

/// 同步回调，即rust调用js/ts/arkts的同步函数，带String入参
#[cfg_attr(feature = "napi", napi)]
pub fn get_version_name(js_function: Function<String, String>) -> String {
    let version_name = js_function.call("input arg".to_string()).unwrap();
    version_name
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug)]
pub struct VersionInfoResponseModel {
    pub version_name: String,
    pub version_code: i32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug)]
pub struct VersionInfoRequestModel {
    pub str_param: String,
    pub int_param: i32,
}

/// 同步回调，即rust调用js/ts/arkts的同步函数，带复杂入参
#[cfg_attr(feature = "napi", napi)]
pub fn get_version_info(
    js_function: Function<VersionInfoRequestModel, VersionInfoResponseModel>,
) -> VersionInfoResponseModel {
    let version_info = js_function
        .call(VersionInfoRequestModel {
            str_param: "str_param_value".to_string(),
            int_param: 100,
        })
        .unwrap();
    version_info
}

/// 异步回调，即rust调用js/ts/arkts的异步函数，无入参
#[cfg_attr(feature = "napi", napi)]
pub async fn get_version_name_async(js_promise: Promise<String>) -> String {
    let version_name = js_promise.await.unwrap();
    version_name
}

/// 异步回调，即rust调用js/ts/arkts的异步函数，带复杂出参
#[cfg_attr(feature = "napi", napi)]
pub async fn get_version_info_async(
    js_promise: Promise<VersionInfoResponseModel>,
) -> VersionInfoResponseModel {
    let version_info = js_promise.await.unwrap();
    version_info
}

/// 异步回调获取version_info，即rust调用js/ts/arkts的异步函数，带复杂入参和出参
#[cfg_attr(feature = "napi", napi)]
pub async fn get_version_info_async_with_input(
    tsfn: ThreadsafeFunction<VersionInfoRequestModel, Promise<VersionInfoResponseModel>>,
) -> Result<VersionInfoResponseModel> {
    let request_model = VersionInfoRequestModel {
        str_param: "str_param_value".to_string(),
        int_param: 104,
    };

    let version_info = tsfn.call_async(Ok(request_model)).await?.await?;
    let mut tsfn_ref_lock = TSFN_REF.write().await;
    *tsfn_ref_lock = Some(tsfn);
    // return Err(Error::new(
    //     Status::FunctionExpected,
    //     "test error".to_string()
    // ));
    Ok(version_info)
}

