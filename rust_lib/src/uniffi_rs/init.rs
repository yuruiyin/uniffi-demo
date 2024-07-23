/// 初始化，如从kotlin传一个callback进来，然后rust就通过这个callback来调用kotlin进而获取到原生平台侧的能力
use std::sync::OnceLock;
use std::thread;

use crate::common::{AppConfig, IS_INITED};
use crate::uniffi_rs::error::UniffiError;
use async_trait::async_trait;
use tokio::time::sleep;

/// 获取原生平台（Android, iOS）的相关能力
#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
#[async_trait]
pub trait AppDelegate: Send + Sync {
    /// 获取原生平台的相关配置
    async fn get_app_config(&self) -> AppConfig;

    /// 获取当前页面（字符串形式）
    async fn get_current_page(&self) -> String;

    /// 打印日志
    async fn log_d(&self, tag: String, message: String);

    /// 获取格式化后的系统时间
    async fn get_system_time(&self) -> String;
}

/// 用于存放原生侧new出来的对象
pub struct AppDelegateManager {
    delegate: Box<dyn AppDelegate>,
}

impl AppDelegateManager {
    pub async fn new(
        delegate: Option<Box<dyn AppDelegate>>,
    ) -> Result<&'static AppDelegateManager, UniffiError> {
        if delegate.is_none() && !*IS_INITED.read().await {
            // 如果delegate为空，且还没有初始化过，则报错
            return Err(UniffiError::InitError("delegate is none and rust has not been initialized".to_string()));
        }

        static INSTANCE: OnceLock<AppDelegateManager> = OnceLock::new();
        Ok(INSTANCE.get_or_init(|| AppDelegateManager {
            delegate: delegate.expect("delegate must be provided"),
        }))
    }
}

// 原生注册
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub async fn register(delegate: Box<dyn AppDelegate>) -> Result<(), UniffiError> {
    if *IS_INITED.read().await {
        return Err(UniffiError::InitError("rust has been inited".to_string()));
    }

    AppDelegateManager::new(Some(delegate)).await?;
    let mut result = IS_INITED.write().await;
    *result = true;

    thread::spawn(move || {
        // 定时获取原生平台的相关配置
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                let app_delegate_manager = AppDelegateManager::new(None).await.unwrap();

                // 1. 原生平台的相关配置
                let app_config = app_delegate_manager.delegate.get_app_config().await;
                let app_config_log = format!("app_config: {:?}", app_config);
                app_delegate_manager
                    .delegate
                    .log_d("rust".to_string(), app_config_log)
                    .await;

                // 2. 当前page
                let current_page = app_delegate_manager.delegate.get_current_page().await;
                let current_page_log = format!("current_page: {:?}", current_page);
                app_delegate_manager
                    .delegate
                    .log_d("rust".to_string(), current_page_log)
                    .await;

                // 3. 系统时间
                let system_time = app_delegate_manager.delegate.get_system_time().await;
                let system_time_log = format!("system_time: {:?}", system_time);
                app_delegate_manager
                    .delegate
                    .log_d("rust".to_string(), system_time_log)
                    .await;

                sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    });

    Ok(())
}
