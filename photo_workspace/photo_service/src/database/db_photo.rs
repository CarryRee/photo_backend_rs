use crate::model::photo_model::PhotoModel;
use common_lib::model::db;
use sqlx::{Pool, MySql};
use axum::http::StatusCode;

/***
 * 查询图片
 */
pub async fn query_photos(
    pool: &Pool<MySql>,
    page_index: i64,
    page_size: i64,
) -> Result<(Vec<PhotoModel>, i64), (StatusCode, String)> {

    tracing::info!("db:query_photos");

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

    // 总数
    let count = sqlx::query_as!(db::Count, "SELECT COUNT(1) AS count FROM photo")
        .fetch_one(&mut *transaction)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok((photos, count.count))
}

/***
 * 插入或更新图片信息
 */
pub async fn insert_or_update_photo (
    pool: &Pool<MySql>,
    photo: &PhotoModel
) -> Result<(), (StatusCode, String)> {

    tracing::info!("db:insert_or_update_photo");

    let _ = sqlx::query!(
"REPLACE INTO photo (id, user_id, photo_path, remark, update_time)
 VALUES (?, ?, ?, ?, curtime())", photo.id, photo.user_id, photo.photo_path, photo.remark)
    .fetch_all(pool)
    .await;

    Ok(())
}

pub async fn delete_photos (
    pool: &Pool<MySql>,
    photo_ids: &Vec<i32>,
) -> Result<(), (StatusCode, String)> {
    tracing::info!("db:delete_photos");

    let params = format!("?{}", ", ?".repeat(photo_ids.len()-1));

    let sql = format!("DELETE FROM photo WHERE id IN ({})", params);
    let mut query = sqlx::query(&sql);

    for id in photo_ids {
        println!("{}", id);
        query = query.bind(id);
    }

    let _ = query.fetch_all(pool).await;
    
    Ok(())
}