use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/*
    类型对应关系：
    https://docs.rs/sqlx/0.6.2/sqlx/mysql/types/index.html
*/

#[derive(Debug, Serialize)]
pub struct PhotoModel {
    pub id: Option<i32>,
    pub user_id: Option<String>,
    // 对应数据库的, 如果是非必须的话，需要用 Option
    pub photo_path: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    // 非必要参数用 Option
    pub page_size: Option<i64>,
    pub page_index: Option<i64>,
}