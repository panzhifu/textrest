use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde_json;

use crate::database::data_base::{bool_to_int, int_to_bool, open_db_with_schema};
use crate::models::book_source::BookSource;

/// 负责 `book_sources` 表的数据库操作
pub struct BookSourceDatabase {
    conn: Connection,
}

impl BookSourceDatabase {
    /// 打开（或创建）指定路径的 SQLite 数据库，并确保 `book_sources` 表已创建。
    pub fn new<P: AsRef<Path>>(path: P) -> rusqlite::Result<Self> {
        let conn = open_db_with_schema(path, Self::init_schema)?;
        Ok(Self { conn })
    }

    fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(
            r#"
CREATE TABLE IF NOT EXISTS book_sources (
    book_source_url TEXT PRIMARY KEY,
    book_source_name TEXT NOT NULL,
    book_source_group TEXT,
    book_source_type INTEGER NOT NULL DEFAULT 0,
    book_url_pattern TEXT,
    custom_order INTEGER DEFAULT 0,
    enabled INTEGER DEFAULT 1,
    enabled_explore INTEGER DEFAULT 0,
    js_lib TEXT,
    enabled_cookie_jar INTEGER DEFAULT 0,
    concurrent_rate TEXT,
    header TEXT,
    login_url TEXT,
    login_ui TEXT,
    login_check_js TEXT,
    cover_decode_js TEXT,
    book_source_comment TEXT,
    variable_comment TEXT,
    last_update_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    respond_time INTEGER DEFAULT 0,
    weight INTEGER DEFAULT 0,
    explore_url TEXT,
    explore_screen TEXT,
    rule_explore TEXT,
    search_url TEXT,
    search_rule TEXT,
    book_info_rule TEXT,
    toc_rule TEXT,
    content_rule TEXT,
    review_rule TEXT
);

CREATE INDEX IF NOT EXISTS idx_book_sources_name ON book_sources(book_source_name);
CREATE INDEX IF NOT EXISTS idx_book_sources_group ON book_sources(book_source_group);
CREATE INDEX IF NOT EXISTS idx_book_sources_type ON book_sources(book_source_type);
CREATE INDEX IF NOT EXISTS idx_book_sources_enabled ON book_sources(enabled);
CREATE INDEX IF NOT EXISTS idx_book_sources_order ON book_sources(custom_order);
CREATE INDEX IF NOT EXISTS idx_book_sources_update ON book_sources(last_update_time);
CREATE INDEX IF NOT EXISTS idx_book_sources_weight ON book_sources(weight);
            "#,
        )?;
        Ok(())
    }

    /// 插入或更新书源。
    pub fn upsert_book_source(&self, src: &BookSource) -> rusqlite::Result<()> {
        self.conn.execute(
            r#"
INSERT INTO book_sources (
    book_source_url, book_source_name, book_source_group, book_source_type,
    book_url_pattern, custom_order, enabled, enabled_explore, js_lib,
    enabled_cookie_jar, concurrent_rate, header, login_url, login_ui,
    login_check_js, cover_decode_js, book_source_comment, variable_comment,
    last_update_time, respond_time, weight, explore_url, explore_screen,
    rule_explore, search_url, search_rule, book_info_rule, toc_rule,
    content_rule, review_rule
) VALUES (
    ?1, ?2, ?3, ?4,
    ?5, ?6, ?7, ?8, ?9,
    ?10, ?11, ?12, ?13, ?14,
    ?15, ?16, ?17, ?18,
    ?19, ?20, ?21, ?22, ?23,
    ?24, ?25, ?26, ?27, ?28,
    ?29, ?30
)
ON CONFLICT(book_source_url) DO UPDATE SET
    book_source_name = excluded.book_source_name,
    book_source_group = excluded.book_source_group,
    book_source_type = excluded.book_source_type,
    book_url_pattern = excluded.book_url_pattern,
    custom_order = excluded.custom_order,
    enabled = excluded.enabled,
    enabled_explore = excluded.enabled_explore,
    js_lib = excluded.js_lib,
    enabled_cookie_jar = excluded.enabled_cookie_jar,
    concurrent_rate = excluded.concurrent_rate,
    header = excluded.header,
    login_url = excluded.login_url,
    login_ui = excluded.login_ui,
    login_check_js = excluded.login_check_js,
    cover_decode_js = excluded.cover_decode_js,
    book_source_comment = excluded.book_source_comment,
    variable_comment = excluded.variable_comment,
    last_update_time = excluded.last_update_time,
    respond_time = excluded.respond_time,
    weight = excluded.weight,
    explore_url = excluded.explore_url,
    explore_screen = excluded.explore_screen,
    rule_explore = excluded.rule_explore,
    search_url = excluded.search_url,
    search_rule = excluded.search_rule,
    book_info_rule = excluded.book_info_rule,
    toc_rule = excluded.toc_rule,
    content_rule = excluded.content_rule,
    review_rule = excluded.review_rule
            "#,
            params![
                src.book_source_url,
                src.book_source_name,
                src.book_source_group,
                src.book_source_type,
                src.book_url_pattern,
                src.custom_order,
                bool_to_int(src.enabled),
                bool_to_int(src.enabled_explore),
                src.js_lib,
                bool_to_int(src.enabled_cookie_jar),
                src.concurrent_rate,
                src.header,
                src.login_url,
                src.login_ui,
                src.login_check_js,
                src.cover_decode_js,
                src.book_source_comment,
                src.variable_comment,
                src.last_update_time,
                src.respond_time,
                src.weight,
                src.explore_url,
                src.explore_screen,
                src
                    .rule_explore
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                src.search_url,
                src
                    .search_rule
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                src
                    .book_info_rule
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                src
                    .toc_rule
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                src
                    .content_rule
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                src
                    .review_rule
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
            ],
        )?;
        Ok(())
    }

    /// 根据 url 查询书源。
    pub fn get_book_source(&self, url: &str) -> rusqlite::Result<Option<BookSource>> {
        self.conn
            .query_row(
                r#"SELECT
    book_source_url, book_source_name, book_source_group, book_source_type,
    book_url_pattern, custom_order, enabled, enabled_explore, js_lib,
    enabled_cookie_jar, concurrent_rate, header, login_url, login_ui,
    login_check_js, cover_decode_js, book_source_comment, variable_comment,
    last_update_time, respond_time, weight, explore_url, explore_screen,
    rule_explore, search_url, search_rule, book_info_rule, toc_rule,
    content_rule, review_rule
FROM book_sources
WHERE book_source_url = ?1
                "#,
                params![url],
                |row| row_to_book_source(row),
            )
            .optional()
    }

    /// 获取所有启用的书源（按权重、排序字段排序）。
    pub fn list_enabled_sources(&self) -> rusqlite::Result<Vec<BookSource>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT
    book_source_url, book_source_name, book_source_group, book_source_type,
    book_url_pattern, custom_order, enabled, enabled_explore, js_lib,
    enabled_cookie_jar, concurrent_rate, header, login_url, login_ui,
    login_check_js, cover_decode_js, book_source_comment, variable_comment,
    last_update_time, respond_time, weight, explore_url, explore_screen,
    rule_explore, search_url, search_rule, book_info_rule, toc_rule,
    content_rule, review_rule
FROM book_sources
WHERE enabled = 1
ORDER BY weight DESC, custom_order ASC, last_update_time DESC
            "#,
        )?;

        let rows = stmt.query_map([], |row| row_to_book_source(row))?;
        let mut result = Vec::new();
        for item in rows {
            result.push(item?);
        }
        Ok(result)
    }

    /// 获取所有书源（按权重、排序字段排序）。
    pub fn list_all_sources(&self) -> rusqlite::Result<Vec<BookSource>> {
        let mut stmt = self.conn.prepare(
            r#"SELECT
    book_source_url, book_source_name, book_source_group, book_source_type,
    book_url_pattern, custom_order, enabled, enabled_explore, js_lib,
    enabled_cookie_jar, concurrent_rate, header, login_url, login_ui,
    login_check_js, cover_decode_js, book_source_comment, variable_comment,
    last_update_time, respond_time, weight, explore_url, explore_screen,
    rule_explore, search_url, search_rule, book_info_rule, toc_rule,
    content_rule, review_rule
FROM book_sources
ORDER BY weight DESC, custom_order ASC, last_update_time DESC
            "#,
        )?;

        let rows = stmt.query_map([], |row| row_to_book_source(row))?;
        let mut result = Vec::new();
        for item in rows {
            result.push(item?);
        }
        Ok(result)
    }

    /// 删除书源。
    pub fn delete_book_source(&self, url: &str) -> rusqlite::Result<usize> {
        self.conn
            .execute("DELETE FROM book_sources WHERE book_source_url = ?1", params![url])
    }
}

fn row_to_book_source(row: &Row) -> rusqlite::Result<BookSource> {
    // 处理 last_update_time 字段，支持多种类型
    let last_update_time = match row.get::<_, Option<i64>>(18) {
        Ok(Some(timestamp)) => {
            // 如果是整数，假设是 Unix 时间戳，转换为 RFC3339 格式
            match chrono::DateTime::from_timestamp(timestamp, 0) {
                Some(dt) => dt.to_rfc3339(),
                None => chrono::Local::now().to_rfc3339(),
            }
        },
        Ok(None) => chrono::Local::now().to_rfc3339(),
        Err(_) => {
            // 如果不是整数，尝试作为字符串获取
            match row.get::<_, String>(18) {
                Ok(s) => s,
                Err(_) => chrono::Local::now().to_rfc3339(),
            }
        },
    };

    Ok(BookSource {
        book_source_url: row.get(0)?,
        book_source_name: row.get(1)?,
        book_source_group: row.get(2)?,
        book_source_type: row.get(3)?,
        book_url_pattern: row.get(4)?,
        custom_order: row.get(5)?,
        enabled: int_to_bool(row.get(6)?),
        enabled_explore: int_to_bool(row.get(7)?),
        js_lib: row.get(8)?,
        enabled_cookie_jar: int_to_bool(row.get(9)?),
        concurrent_rate: row.get(10)?,
        header: row.get(11)?,
        login_url: row.get(12)?,
        login_ui: row.get(13)?,
        login_check_js: row.get(14)?,
        cover_decode_js: row.get(15)?,
        book_source_comment: row.get(16)?,
        variable_comment: row.get(17)?,
        last_update_time,
        respond_time: row.get(19)?,
        weight: row.get(20)?,
        explore_url: row.get(21)?,
        explore_screen: row.get(22)?,
        rule_explore: {
            let v: Option<String> = row.get(23)?;
            match v {
                Some(s) if !s.is_empty() => serde_json::from_str(&s).ok(),
                _ => None,
            }
        },
        search_url: row.get(24)?,
        search_rule: {
            let v: Option<String> = row.get(25)?;
            match v {
                Some(s) if !s.is_empty() => serde_json::from_str(&s).ok(),
                _ => None,
            }
        },
        book_info_rule: {
            let v: Option<String> = row.get(26)?;
            match v {
                Some(s) if !s.is_empty() => serde_json::from_str(&s).ok(),
                _ => None,
            }
        },
        toc_rule: {
            let v: Option<String> = row.get(27)?;
            match v {
                Some(s) if !s.is_empty() => serde_json::from_str(&s).ok(),
                _ => None,
            }
        },
        content_rule: {
            let v: Option<String> = row.get(28)?;
            match v {
                Some(s) if !s.is_empty() => serde_json::from_str(&s).ok(),
                _ => None,
            }
        },
        review_rule: {
            let v: Option<String> = row.get(29)?;
            match v {
                Some(s) if !s.is_empty() => serde_json::from_str(&s).ok(),
                _ => None,
            }
        },
    })
}

