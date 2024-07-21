/// 初始化，如从kotlin传一个callback进来，然后rust就通过这个callback来调用kotlin进而获取到android平台侧的能力
use std::sync::OnceLock;
use std::thread;

use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::error::GDError;

lazy_static! {
    /// 是否已经初始化过了，防止多次初始化
    pub static ref IS_INITED: RwLock<bool> = RwLock::new(false);
}

/// android平台的相关配置
#[derive(uniffi::Record, Debug)]
pub struct AndroidConfig {
    pub version: i32,
    pub brand: String,
    pub model: String,
}

/// 获取android平台的相关能力
#[uniffi::export(callback_interface)]
#[async_trait]
pub trait AndroidDelegate: Send + Sync {
    /// 获取android平台的相关配置
    async fn get_android_config(&self) -> AndroidConfig;

    /// 获取当前activity（字符串形式）
    async fn get_current_activity(&self) -> String;

    /// 打印debug日志
    async fn log_d(&self, tag: String, message: String);

    /// 获取格式化后的系统时间
    async fn get_system_time(&self) -> String;
}

/// 用于存放kotlin侧new出来的对象
pub struct AndroidDelegateManager {
    delegate: Box<dyn AndroidDelegate>,
}

impl AndroidDelegateManager {
    pub async fn new(delegate: Option<Box<dyn AndroidDelegate>>) -> Result<&'static AndroidDelegateManager, GDError> {
        if delegate.is_none() && !*IS_INITED.read().await {
            // 如果delegate为空，且还没有初始化过，则报错
            return Err(GDError::InitError("delegate is none and rust is not inited".to_string()));
        }

        static INSTANCE: OnceLock<AndroidDelegateManager> = OnceLock::new();
        Ok(INSTANCE.get_or_init(|| {
            AndroidDelegateManager {
                delegate: delegate.expect("delegate must be provided"),
            }
        }))
    }
}

// 原生注册
#[uniffi::export]
pub async fn register(delegate: Box<dyn AndroidDelegate>) -> Result<(), GDError> {
    // return Err(GDError::InitError("custom init error".to_string()));
    if *IS_INITED.read().await {
        return Err(GDError::InitError("Already inited".to_string()));
    }

    AndroidDelegateManager::new(Some(delegate)).await?;
    let mut result = IS_INITED.write().await;
    *result = true;

    thread::spawn(move || {
        // 定时获取android平台的相关配置
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                let android_delegate_manager = AndroidDelegateManager::new(None).await.unwrap();

                // 1. android平台的相关配置
                let android_config = android_delegate_manager.delegate.get_android_config().await;
                let android_config_log = format!("android_config: {:?}", android_config);
                android_delegate_manager.delegate.log_d("rust".to_string(), android_config_log).await;

                // 2. 当前activity
                let current_activity = android_delegate_manager.delegate.get_current_activity().await;
                let current_activity_log = format!("current_activity: {:?}", current_activity);
                android_delegate_manager.delegate.log_d("rust".to_string(), current_activity_log).await;

                // 3. 系统时间
                let system_time = android_delegate_manager.delegate.get_system_time().await;
                let system_time_log = format!("system_time: {:?}", system_time);
                android_delegate_manager.delegate.log_d("rust".to_string(), system_time_log).await;

                sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    });

    Ok(())
}
