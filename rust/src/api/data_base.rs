use std::sync::{Arc, Mutex, OnceLock};

use anyhow::Result;
use flutter_rust_bridge::frb;

use crate::database::DatabaseManager;

#[frb(ignore)]
static DB_POOL: OnceLock<Arc<Mutex<DatabaseManager>>> = OnceLock::new();

/// 初始化全局数据库连接池
/// 应在应用启动时调用一次
pub fn init_database() -> Result<()> {
    let db = Arc::new(Mutex::new(DatabaseManager::new()?));
    DB_POOL.set(db).map_err(|_| anyhow::anyhow!("数据库已初始化"))?;
    Ok(())
}

/// 获取全局数据库管理器
#[frb(ignore)]
pub fn get_db() -> Arc<Mutex<DatabaseManager>> {
    DB_POOL.get()
        .expect("数据库未初始化，请先调用 init_database()")
        .clone()
}

/// 检查数据库是否已初始化
pub fn is_initialized() -> bool {
    DB_POOL.get().is_some()
}
