use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::database::data_base::open_db_with_schema;
use crate::models::ReadRecord;

/// 负责 `read_records` 表的数据库操作
pub struct ReadRecordDatabase {
    conn: Connection,
}

impl ReadRecordDatabase {
    /// 打开（或创建）指定路径的 SQLite 数据库，并确保 `read_records` 表已创建。
    pub fn new<P: AsRef<Path>>(path: P) -> rusqlite::Result<Self> {
        let conn = open_db_with_schema(path, Self::init_schema)?;
        Ok(Self { conn })
    }

    fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(
            r#"
CREATE TABLE IF NOT EXISTS read_records (
    book_id TEXT PRIMARY KEY,
    dur_chapter_index INTEGER DEFAULT 0,
    dur_chapter_pos INTEGER DEFAULT 0,
    last_chapter_index INTEGER DEFAULT 0,
    last_chapter_pos INTEGER DEFAULT 0,
    total_read_time INTEGER DEFAULT 0,
    last_read_time DATETIME
);

CREATE INDEX IF NOT EXISTS idx_read_records_time ON read_records(last_read_time);
            "#,
        )?;
        Ok(())
    }

    /// 插入或更新阅读记录。
    pub fn upsert_record(&self, record: &ReadRecord) -> rusqlite::Result<()> {
        self.conn.execute(
            r#"
INSERT INTO read_records (
    book_id, dur_chapter_index, dur_chapter_pos,
    last_chapter_index, last_chapter_pos,
    total_read_time, last_read_time
) VALUES (
    ?1, ?2, ?3,
    ?4, ?5,
    ?6, ?7
)
ON CONFLICT(book_id) DO UPDATE SET
    dur_chapter_index = excluded.dur_chapter_index,
    dur_chapter_pos = excluded.dur_chapter_pos,
    last_chapter_index = excluded.last_chapter_index,
    last_chapter_pos = excluded.last_chapter_pos,
    total_read_time = excluded.total_read_time,
    last_read_time = excluded.last_read_time
            "#,
            params![
                record.book_id,
                record.dur_chapter_index,
                record.dur_chapter_pos,
                record.last_chapter_index,
                record.last_chapter_pos,
                record.total_read_time,
                record.last_read_time,
            ],
        )?;
        Ok(())
    }

    /// 查询某本书的阅读记录。
    pub fn get_record(&self, book_id: &str) -> rusqlite::Result<Option<ReadRecord>> {
        self.conn
            .query_row(
                r#"SELECT
    book_id, dur_chapter_index, dur_chapter_pos,
    last_chapter_index, last_chapter_pos,
    total_read_time, last_read_time
FROM read_records
WHERE book_id = ?1
                "#,
                params![book_id],
                |row| row_to_record(row),
            )
            .optional()
    }

    /// 删除指定书籍的阅读记录。
    pub fn delete_record(&self, book_id: &str) -> rusqlite::Result<()> {
        self.conn
            .execute("DELETE FROM read_records WHERE book_id = ?1", params![book_id])?;
        Ok(())
    }

    /// 按最后阅读时间倒序列出所有记录。
    pub fn list_recent(&self) -> rusqlite::Result<Vec<ReadRecord>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT
    book_id, dur_chapter_index, dur_chapter_pos,
    last_chapter_index, last_chapter_pos,
    total_read_time, last_read_time
FROM read_records
ORDER BY last_read_time DESC
            "#,
        )?;

        let rows = stmt.query_map([], |row| row_to_record(row))?;
        let mut result = Vec::new();
        for item in rows {
            result.push(item?);
        }
        Ok(result)
    }

    /// 根据book_id更新阅读记录
    pub fn update_record_by_book_id(&self, book_id: &str, record: &ReadRecord) -> rusqlite::Result<()> {
        self.conn.execute(
            r#"UPDATE read_records SET
    dur_chapter_index = ?1,
    dur_chapter_pos = ?2,
    last_chapter_index = ?3,
    last_chapter_pos = ?4,
    total_read_time = ?5,
    last_read_time = ?6
    WHERE book_id = ?7
    "#,
            params![
                record.dur_chapter_index,
                record.dur_chapter_pos,
                record.last_chapter_index,
                record.last_chapter_pos,
                record.total_read_time,
                record.last_read_time,
                book_id,
            ],
        )?;
        Ok(())
    }

    /// 根据book_id获取阅读记录
    pub fn get_record_by_book_id(&self, book_id: &str) -> rusqlite::Result<Option<ReadRecord>> {
        self.get_record(book_id)
    }

    /// 删除阅读记录
    pub fn delete_record_by_book_id(&self, book_id: &str) -> rusqlite::Result<()> {
        self.delete_record(book_id)
    }
}

/// 将 SQLite 行转换为 ReadRecord 结构体
fn row_to_record(row: &Row) -> rusqlite::Result<ReadRecord> {
    Ok(ReadRecord {
        book_id: row.get(0)?,
        dur_chapter_index: row.get(1)?,
        dur_chapter_pos: row.get(2)?,
        last_chapter_index: row.get(3)?,
        last_chapter_pos: row.get(4)?,
        total_read_time: row.get(5)?,
        last_read_time: row.get(6)?,
    })
}

