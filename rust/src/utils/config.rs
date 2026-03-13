use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::config::Config;
use crate::utils::file::ensure_config_dir;

/// 配置文件名称
const CONFIG_FILE_NAME: &str = "config.json";

/// 保存配置到文件
pub fn save_config(app_name: &str, config: &Config) -> Result<()> {
    let config_dir = ensure_config_dir(app_name)?;
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    
    let config_json = serde_json::to_string_pretty(config)
        .context("Failed to serialize config")?;
    
    fs::write(&config_path, config_json)
        .context(format!("Failed to write config file: {:?}", config_path))?;
    
    Ok(())
}

/// 从文件加载配置
pub fn load_config(app_name: &str) -> Result<Config> {
    let config_dir = ensure_config_dir(app_name)?;
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    
    if !config_path.exists() {
        // 配置文件不存在，返回默认配置
        return Ok(Config::default());
    }
    
    let config_json = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file: {:?}", config_path))?;
    
    let config = serde_json::from_str(&config_json)
        .context("Failed to deserialize config")?;
    
    Ok(config)
}

/// 获取配置文件路径
pub fn get_config_path(app_name: &str) -> Result<PathBuf> {
    let config_dir = ensure_config_dir(app_name)?;
    Ok(config_dir.join(CONFIG_FILE_NAME))
}
