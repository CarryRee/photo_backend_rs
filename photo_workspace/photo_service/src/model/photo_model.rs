use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PhotoModel {
    pub id: i32,
    pub user_id: String,
    // 对应数据库的, 如果是非必须的话，需要用 Option
    pub photo_path: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}


#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    // 非必要参数用 Option
    pub page_size: Option<i64>,
    pub page_index: Option<i64>,
}