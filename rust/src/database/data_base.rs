use std::path::{Path, PathBuf};

use anyhow::Result;
use rusqlite::{Connection, Result as SqlResult};

use crate::database::{BookDatabase, BookSourceDatabase, ChapterDatabase, ReadRecordDatabase};
use crate::utils::ensure_config_dir;

/// 默认应用名，决定配置与数据库目录位置。
const DEFAULT_APP_NAME: &str = "textrest";

/// 统一管理整个应用的数据库：
/// - 决定数据库文件所在路径（使用跨平台配置目录）；
/// - 提供对各个表对应 Database 的访问。
pub struct DatabaseManager {
    pub books: BookDatabase,
    pub chapters: ChapterDatabase,
    pub book_sources: BookSourceDatabase,
    pub read_records: ReadRecordDatabase,
    db_dir: PathBuf,
}

impl DatabaseManager {
    /// 使用默认应用名初始化数据库管理器。
    pub fn new() -> Result<Self> {
        Self::with_app_name(DEFAULT_APP_NAME)
    }

    /// 指定应用名初始化数据库管理器。
    pub fn with_app_name(app_name: &str) -> Result<Self> {
        let db_dir = ensure_config_dir(app_name)?.join("db");

        let books = BookDatabase::new(db_dir.join("books.db"))?;
        let chapters = ChapterDatabase::new(db_dir.join("chapters.db"))?;
        let book_sources = BookSourceDatabase::new(db_dir.join("book_sources.db"))?;
        let read_records = ReadRecordDatabase::new(db_dir.join("read_records.db"))?;

        Ok(Self {
            books,
            chapters,
            book_sources,
            read_records,
            db_dir,
        })
    }

    /// 返回数据库目录路径，方便调试或备份。
    pub fn db_dir(&self) -> &Path {
        &self.db_dir
    }
}



/// 打开指定路径的 SQLite 数据库并初始化表结构（仅在本 crate 内部复用）。
pub(crate) fn open_db_with_schema<P, F>(path: P, init: F) -> SqlResult<Connection>
where
    P: AsRef<Path>,
    F: FnOnce(&Connection) -> SqlResult<()>,
{
    let conn = Connection::open(path)?;
    init(&conn)?;
    Ok(conn)
}

/// 将 bool 转成 SQLite 中常用的 0/1 整数。
#[inline]
pub(crate) fn bool_to_int(v: bool) -> i32 {
    if v {
        1
    } else {
        0
    }
}

/// 将 SQLite 中的 0/1 整数转换为 bool。
#[inline]
pub(crate) fn int_to_bool(v: i32) -> bool {
    v != 0
}


