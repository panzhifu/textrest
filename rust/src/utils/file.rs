use std::fs;
use std::path::PathBuf;

/// 使用 `dirs` crate 获取平台相关的配置根目录：
/// - Windows: %APPDATA%
/// - macOS:  ~/Library/Application Support
/// - Linux:  $XDG_CONFIG_HOME 或 ~/.config
fn base_config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}

/// 获取指定应用的配置目录路径（仅构造路径，不创建目录）。
///
/// 例如应用名为 "textrest" 时：
/// - Windows: %APPDATA%\\textrest
/// - macOS:  ~/Library/Application Support/textrest
/// - Linux:  $XDG_CONFIG_HOME/textrest 或 ~/.config/textrest
pub fn get_config_dir(app_name: &str) -> Option<PathBuf> {
    base_config_dir().map(|base| base.join(app_name))
}

/// 确保配置目录存在（如果不存在则创建），并返回最终路径。
pub fn ensure_config_dir(app_name: &str) -> std::io::Result<PathBuf> {
    let path = get_config_dir(app_name)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "cannot determine config dir"))?;
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

