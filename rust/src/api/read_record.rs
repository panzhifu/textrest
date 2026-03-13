use anyhow::Result;
use flutter_rust_bridge::frb;

use crate::api::data_base::get_db;
use crate::models::ReadRecord;

/// 阅读进度 API
#[frb(opaque)]
pub struct ReadRecordApi;

impl ReadRecordApi {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// 创建阅读进度
    pub fn create_read_record(&self, record: &ReadRecord) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().read_records.upsert_record(record)
            .map_err(|e| anyhow::anyhow!("创建阅读进度失败: {}", e))?;
        Ok(())
    }

    /// 获取阅读进度
    pub fn get_read_record(&self, book_id: &str) -> Result<Option<ReadRecord>> {
        let db = get_db();
        let result = db.lock().unwrap().read_records.get_record(book_id)
            .map_err(|e| anyhow::anyhow!("获取阅读进度失败: {}", e))?;
        Ok(result)
    }

    /// 更新阅读进度
    pub fn update_read_record(&self, record: &ReadRecord) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().read_records.upsert_record(record)
            .map_err(|e| anyhow::anyhow!("更新阅读进度失败: {}", e))?;
        Ok(())
    }

    /// 删除阅读进度
    pub fn delete_read_record(&self, book_id: &str) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().read_records.delete_record(book_id)
            .map_err(|e| anyhow::anyhow!("删除阅读进度失败: {}", e))?;
        Ok(())
    }

    /// 根据book_id更新阅读记录
    pub fn update_record_by_book_id(&self, book_id: &str, record: &ReadRecord) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().read_records.update_record_by_book_id(book_id, record)
            .map_err(|e| anyhow::anyhow!("更新阅读进度失败: {}", e))?;
        Ok(())
    }

    /// 根据book_id获取阅读记录
    pub fn get_record_by_book_id(&self, book_id: &str) -> Result<Option<ReadRecord>> {
        let db = get_db();
        let result = db.lock().unwrap().read_records.get_record_by_book_id(book_id)
            .map_err(|e| anyhow::anyhow!("获取阅读进度失败: {}", e))?;
        Ok(result)
    }

    /// 按最后阅读时间倒序列出所有记录
    pub fn list_recent(&self) -> Result<Vec<ReadRecord>> {
        let db = get_db();
        let result = db.lock().unwrap().read_records.list_recent()
            .map_err(|e| anyhow::anyhow!("获取阅读记录列表失败: {}", e))?;
        Ok(result)
    }

    /// 删除阅读记录
    pub fn delete_record_by_book_id(&self, book_id: &str) -> Result<()> {
        let db = get_db();
        db.lock().unwrap().read_records.delete_record_by_book_id(book_id)
            .map_err(|e| anyhow::anyhow!("删除阅读进度失败: {}", e))?;
        Ok(())
    }
}


