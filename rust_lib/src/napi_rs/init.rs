// 鸿蒙平台初始化（注册），因为它的ffi采用的是napi，而napi与uniffi的callback差别很大，比如napi不支持trait（只能使用JsFunction来实现回调），
// 而uniffi支持trait，因此，这里需要单独处理

use std::sync::OnceLock;
use std::thread;

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::{Promise, Result};
use napi_ohos::Error;
use napi_ohos::threadsafe_function::ThreadsafeFunction;
use tokio::time::sleep;

use crate::common::{AppConfig, IS_INITED};
use crate::log::{AppLog, AppLogLevel};
use crate::napi_rs::js_callback::{TSFN_REF, VersionInfoRequestModel, VersionInfoResponseModel};

// lazy_static! {
//     pub static ref LOG_TSFN_REF: RwLock<Option<ThreadsafeFunction<AppLog, Promise<()>>>> = RwLock::new(None);
// }

// 获取鸿蒙原生平台的相关能力，基本上要保证和 Android/iOS 暴露的能力保持一致，详见 `AppDelegate` 的定义
struct NapiAppDelegate {
    /// 获取原生平台的相关配置, () => AppConfig
    pub app_config_fn: ThreadsafeFunction<(), Promise<AppConfig>>,

    /// 获取当前页面（字符串形式）, () => String
    pub current_page_fn: ThreadsafeFunction<(), Promise<String>>,

    /// 打印日志, (AppLog) => ()
    pub log_fn: ThreadsafeFunction<AppLog, Promise<()>>,

    /// 获取格式化后的系统时间, () => String
    pub system_time_fn: ThreadsafeFunction<(), Promise<String>>,
}

pub struct NapiAppDelegateManager {
    delegate: NapiAppDelegate,
}

impl NapiAppDelegateManager {
    pub async fn new(
        delegate: Option<NapiAppDelegate>,
    ) -> Result<&'static NapiAppDelegateManager> {
        if delegate.is_none() && !*IS_INITED.read().await {
            // 如果delegate为空，且还没有初始化过，则报错
            return Err(Error::new(
                napi_ohos::Status::GenericFailure,
                "delegate is none and rust has not been initialized",
            ));
        }

        static INSTANCE: OnceLock<NapiAppDelegateManager> = OnceLock::new();
        Ok(INSTANCE.get_or_init(|| NapiAppDelegateManager {
            delegate: delegate.expect("delegate must be provided"),
        }))
    }

    /// 获取原生平台的相关配置
    pub async fn get_app_config(&self) -> Result<AppConfig> {
        self.delegate.app_config_fn.call_async(Ok(())).await?.await
    }

    /// 获取当前页面（字符串形式）
    pub async fn get_current_page(&self) -> Result<String> {
        self.delegate.current_page_fn.call_async(Ok(())).await?.await
    }

    /// 打印日志
    pub async fn log(&self, level: AppLogLevel, tag: String, message: String) -> Result<()> {
        self.delegate.log_fn.call_async(Ok(AppLog { level, tag, message })).await?.await
    }

    /// 获取格式化后的系统时间
    pub async fn get_system_time(&self) -> Result<String> {
        self.delegate.system_time_fn.call_async(Ok(())).await?.await
    }
}

async fn get_version_info(request_model: VersionInfoRequestModel) -> Result<VersionInfoResponseModel> {
    let response_model = TSFN_REF.read().await.as_ref().unwrap().call_async(Ok(request_model)).await?.await?;
    Ok(response_model)
}

/// 原生注册
/// * `app_config_fn` 获取原生平台的相关配置
/// * `current_page_fn` 获取当前页面（字符串形式）
/// * `log_func` 打印日志
/// * `system_time_fn` 获取格式化后的系统时间
#[cfg_attr(feature = "napi", napi)]
pub async fn register(
    app_config_fn: ThreadsafeFunction<(), Promise<AppConfig>>,
    current_page_fn: ThreadsafeFunction<(), Promise<String>>,
    log_fn: ThreadsafeFunction<AppLog, Promise<()>>,
    system_time_fn: ThreadsafeFunction<(), Promise<String>>,
) -> Result<()> {
    if *IS_INITED.read().await {
        return Err(Error::new(
            napi_ohos::Status::GenericFailure,
            "rust has been initialized",
        ));
    }

    NapiAppDelegateManager::new(Some(NapiAppDelegate {
        app_config_fn,
        current_page_fn,
        log_fn,
        system_time_fn,
    })).await?;
    let mut result = IS_INITED.write().await;
    *result = true;

    // 开启一个线程，模拟获取原生平台的相关配置
    thread::spawn(move || {
        // 定时获取原生平台的相关配置
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                // 模拟rust调用鸿蒙原生arkts
                let app_delegate = NapiAppDelegateManager::new(None).await.unwrap();

                // 1. 原生平台的相关配置
                let app_config = app_delegate.get_app_config().await;
                if let Ok(app_config) = app_config {
                    let app_config_log = format!("app_config: {:?}", app_config);
                    app_delegate.log(AppLogLevel::Debug, "rust".to_string(), app_config_log).await.unwrap();
                }

                // 2. 当前page
                let current_page = app_delegate.get_current_page().await;
                if let Ok(current_page) = current_page {
                    let current_page_log = format!("current_page: {:?}", current_page);
                    app_delegate.log(AppLogLevel::Debug, "rust".to_string(), current_page_log).await.unwrap();
                }

                // 3. 系统时间
                let system_time = app_delegate.get_system_time().await;
                if let Ok(system_time) = system_time {
                    let system_time_log = format!("system_time: {:?}", system_time);
                    app_delegate.log(AppLogLevel::Debug, "rust".to_string(), system_time_log).await.unwrap();
                }

                sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    });

    Ok(())
}
