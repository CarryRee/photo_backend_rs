use axum::{extract::{State, Query}, Json};
use sqlx::{Pool, MySql};

use axum::{
    extract::Multipart,
    http::StatusCode,
};

use crate::model::photo_model::{PhotoModel, QueryRequest};
use crate::database::db_photo;
use common_lib::model::response::{Response, Page};

pub async fn get_photos(
    State(pool): State<Pool<MySql>>,
    Query(query_params): Query<QueryRequest>,
) -> Result<Json<Response<Page<PhotoModel>>>, (StatusCode, String)> {

    let index = match query_params.page_index {
        Some(x) if x >= 1 => x,
        _ => 1
    };

    let size = match query_params.page_size {
        Some(x) if x >= 1 => x,
        _ => 20
    };

    let photo_models: (Vec<PhotoModel>, i64) = db_photo::query_photos(&pool, index, size).await?;
    
    let total = photo_models.1;
    let residue = if total % size == 0 {0} else {1};
    let num = total / size + residue;

    let page = Page {
        page_num: num, 
        page_index: index, 
        page_size: size,
        data: photo_models.0,
        total: photo_models.1,
    };
    let response: Response<Page<PhotoModel>> = Response {code:0, message: "success".to_string(), data: page};

    Ok(Json(response))
}


pub async fn upload_photo (
    mut multipart:Multipart
) -> Result<String, (StatusCode, String)> {

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // 原文件名
        let file_name = field.file_name().unwrap().to_string();
        // 文件类型
        let content_type = field.content_type().unwrap().to_string();

        if content_type.starts_with("image/"){
            
        }
        // 原始数据
        let data = field.bytes().await.unwrap();

        tracing::info!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }

    Ok("Ok".to_string())
}