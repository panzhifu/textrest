use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::database::data_base::{int_to_bool, open_db_with_schema};
use crate::models::Chapter;

/// 负责 `chapters` 表的数据库操作
pub struct ChapterDatabase {
    conn: Connection,
}

impl ChapterDatabase {
    /// 打开（或创建）指定路径的 SQLite 数据库，并确保 `chapters` 表已创建。
    pub fn new<P: AsRef<Path>>(path: P) -> rusqlite::Result<Self> {
        let conn = open_db_with_schema(path, Self::init_schema)?;
        Ok(Self { conn })
    }

    fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(
            r#"
CREATE TABLE IF NOT EXISTS chapters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id TEXT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    chapter_index INTEGER NOT NULL,
    word_count INTEGER DEFAULT 0,
    is_vip INTEGER DEFAULT 0,
    UNIQUE(book_id, chapter_index)
);

CREATE INDEX IF NOT EXISTS idx_chapters_book ON chapters(book_id);
CREATE INDEX IF NOT EXISTS idx_chapters_index ON chapters(chapter_index);
            "#,
        )?;
        Ok(())
    }

    /// 插入或更新章节（按 book_id + chapter_index 唯一）。
    pub fn upsert_chapter(&self, chapter: &Chapter) -> rusqlite::Result<()> {
        self.conn.execute(
            r#"
INSERT INTO chapters (
    book_id, title, url, chapter_index, word_count, is_vip
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6
)
ON CONFLICT(book_id, chapter_index) DO UPDATE SET
    title = excluded.title,
    url = excluded.url,
    word_count = excluded.word_count,
    is_vip = excluded.is_vip
            "#,
            params![
                chapter.book_id,
                chapter.title,
                chapter.url,
                chapter.chapter_index,
                chapter.word_count,
                chapter.is_vip as i32,
            ],
        )?;
        Ok(())
    }

    /// 根据 id 查询单个章节。
    pub fn get_chapter(&self, id: u32) -> rusqlite::Result<Option<Chapter>> {
        self.conn
            .query_row(
                r#"SELECT
    id, book_id, title, url, chapter_index, word_count, is_vip
FROM chapters
WHERE id = ?1
                "#,
                params![id],
                |row| row_to_chapter(row),
            )
            .optional()
    }

    /// 查询某本书的所有章节，按 chapter_index 升序。
    pub fn list_chapters_by_book(&self, book_id: &str) -> rusqlite::Result<Vec<Chapter>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT
    id, book_id, title, url, chapter_index, word_count, is_vip
FROM chapters
WHERE book_id = ?1
ORDER BY chapter_index ASC
            "#,
        )?;

        let rows = stmt.query_map(params![book_id], |row| row_to_chapter(row))?;
        let mut result = Vec::new();
        for item in rows {
            result.push(item?);
        }
        Ok(result)
    }

    /// 删除某本书的所有章节。
    pub fn delete_chapters_by_book(&self, book_id: &str) -> rusqlite::Result<usize> {
        self.conn
            .execute("DELETE FROM chapters WHERE book_id = ?1", params![book_id])
    }
}

fn row_to_chapter(row: &Row) -> rusqlite::Result<Chapter> {
    Ok(Chapter {
        id: row.get(0)?,
        book_id: row.get(1)?,
        title: row.get(2)?,
        url: row.get(3)?,
        chapter_index: row.get(4)?,
        word_count: row.get(5)?,
        is_vip: int_to_bool(row.get(6)?),
    })
}

