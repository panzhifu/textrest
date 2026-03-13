use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::database::data_base::open_db_with_schema;
use crate::models::Book;

/// 负责 `books` 表的数据库操作
pub struct BookDatabase {
    conn: Connection,
}

impl BookDatabase {
    /// 打开（或创建）指定路径的 SQLite 数据库，并确保 `books` 表已创建。
    pub fn new<P: AsRef<Path>>(path: P) -> rusqlite::Result<Self> {
        let conn = open_db_with_schema(path, Self::init_schema)?;
        Ok(Self { conn })
    }

    fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(
            r#"
CREATE TABLE IF NOT EXISTS books (
    book_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    kind TEXT,
    cover_url TEXT,
    intro TEXT,
    origin TEXT,
    book_type TEXT NOT NULL,
    word_count INTEGER DEFAULT 0,
    latest_chapter_title TEXT,
    toc_url TEXT,
    book_group TEXT,
    add_time INTEGER NOT NULL DEFAULT 0,
    last_read_time INTEGER,
    isbn TEXT,
    publisher TEXT,
    publish_date INTEGER,
    status TEXT DEFAULT '未知'
);

CREATE INDEX IF NOT EXISTS idx_books_name ON books(name);
CREATE INDEX IF NOT EXISTS idx_books_author ON books(author);
CREATE INDEX IF NOT EXISTS idx_books_origin ON books(origin);
CREATE INDEX IF NOT EXISTS idx_books_type ON books(book_type);
CREATE INDEX IF NOT EXISTS idx_books_group ON books(book_group);
CREATE INDEX IF NOT EXISTS idx_books_last_read ON books(last_read_time);
        "#,
        )?;
        Ok(())
    }

    /// 插入或更新一本书。
    pub fn upsert_book(&self, book: &Book) -> rusqlite::Result<()> {
        self.conn.execute(
            r#"
INSERT INTO books (
    book_id, name, author, kind, cover_url, intro, origin, book_type,
    word_count, latest_chapter_title, toc_url, book_group, add_time,
    last_read_time, isbn, publisher, publish_date, status
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8,
    ?9, ?10, ?11, ?12, ?13,
    ?14, ?15, ?16, ?17, ?18
)
ON CONFLICT(book_id) DO UPDATE SET
    name = excluded.name,
    author = excluded.author,
    kind = excluded.kind,
    cover_url = excluded.cover_url,
    intro = excluded.intro,
    origin = excluded.origin,
    book_type = excluded.book_type,
    word_count = excluded.word_count,
    latest_chapter_title = excluded.latest_chapter_title,
    toc_url = excluded.toc_url,
    book_group = excluded.book_group,
    add_time = excluded.add_time,
    last_read_time = excluded.last_read_time,
    isbn = excluded.isbn,
    publisher = excluded.publisher,
    publish_date = excluded.publish_date,
    status = excluded.status
            "#,
            params![
                book.book_id,
                book.name,
                book.author,
                book.kind,
                book.cover_url,
                book.intro,
                book.origin,
                book.book_type,
                book.word_count,
                book.latest_chapter_title,
                book.toc_url,
                book.book_group,
                book.add_time,
                book.last_read_time,
                book.isbn,
                book.publisher,
                book.publish_date,
                book.status,
            ],
        )?;
        Ok(())
    }

    /// 根据 book_id 查询一本书。
    pub fn get_book(&self, book_id: &str) -> rusqlite::Result<Option<Book>> {
        self.conn
            .query_row(
                r#"SELECT
    book_id, name, author, kind, cover_url, intro, origin, book_type,
    word_count, latest_chapter_title, toc_url, book_group, add_time,
    last_read_time, isbn, publisher, publish_date, status
FROM books
WHERE book_id = ?1
                "#,
                params![book_id],
                |row| row_to_book(row),
            )
            .optional()
    }

    /// 按添加时间倒序列出所有书。
    pub fn list_books(&self) -> rusqlite::Result<Vec<Book>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT
    book_id, name, author, kind, cover_url, intro, origin, book_type,
    word_count, latest_chapter_title, toc_url, book_group, add_time,
    last_read_time, isbn, publisher, publish_date, status
FROM books
ORDER BY add_time DESC
            "#,
        )?;

        let iter = stmt.query_map([], |row| row_to_book(row))?;
        let mut result = Vec::new();
        for item in iter {
            result.push(item?);
        }
        Ok(result)
    }

    /// 删除指定 book_id 的书。
    pub fn delete_book(&self, book_id: &str) -> rusqlite::Result<usize> {
        self.conn
            .execute("DELETE FROM books WHERE book_id = ?1", params![book_id])
    }
}

fn row_to_book(row: &Row) -> rusqlite::Result<Book> {
    Ok(Book {
        book_id: row.get(0)?,
        name: row.get(1)?,
        author: row.get(2)?,
        kind: row.get(3)?,
        cover_url: row.get(4)?,
        intro: row.get(5)?,
        origin: row.get(6)?,
        book_type: row.get(7)?,
        word_count: row.get(8)?,
        latest_chapter_title: row.get(9)?,
        toc_url: row.get(10)?,
        book_group: row.get(11)?,
        add_time: row.get(12)?,
        last_read_time: row.get(13)?,
        isbn: row.get(14)?,
        publisher: row.get(15)?,
        publish_date: row.get(16)?,
        status: row.get(17)?,
    })      
}

