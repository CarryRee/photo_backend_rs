use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response <T: serde::Serialize> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

// 序列化要求加上 Serialize
#[derive(Debug, Serialize)]
pub struct Page<V : serde::Serialize> {
    pub total:i64,
    pub page_index:i64,
    pub page_size:i64,
    pub page_num:i64,
    pub data: Vec<V>,
} 