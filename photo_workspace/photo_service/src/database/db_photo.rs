use crate::model::photo_model::PhotoModel;
use sqlx::{Pool, MySql};
use axum::http::StatusCode;


pub async fn query_photos(
    pool: &Pool<MySql>,
    page_index: u32,
    page_size: u32,
) -> Result<Vec<PhotoModel>, (StatusCode, String)> {

    // 计算偏移量
    let offset = (page_index - 1) * page_size;

    // 开启事务
    let mut transaction = pool.begin().await.unwrap();

    // 分页
    let photos = sqlx::query_as!(PhotoModel,
"SELECT id, user_id, photo_path, remark, create_time, update_time 
FROM photo ORDER BY id LIMIT ? OFFSET ?", page_size, offset)
        .fetch_all(&mut *transaction) 
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok(photos)
}