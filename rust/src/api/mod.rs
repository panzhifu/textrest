pub mod book;
pub mod book_source;
pub mod data_base;
pub mod read_record;
pub mod web_book;

pub use data_base::{init_database, is_initialized};
