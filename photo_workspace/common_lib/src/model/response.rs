use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response <T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 序列化要求加上 Serialize
#[derive(Debug, Serialize, Deserialize)]
pub struct Page<V> {
    pub total:i64,
    pub page_index:i64,
    pub page_size:i64,
    pub page_num:i64,
    pub data: Vec<V>,
} 



