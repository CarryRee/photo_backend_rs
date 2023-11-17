use axum::{extract::{State, Query}, Json};
use sqlx::{Pool, MySql};
use std::env;
use std::path::Path;
use uuid::Uuid;

use axum::{
    extract::Multipart,
    http::StatusCode,
};

use crate::model::photo_model::{PhotoModel, QueryRequest, QueryCollect};
use crate::database::db_photo;
use common_lib::model::response::{Response, Page};

/**
    获取图片接口
*/
pub async fn get_photos(
    State(pool): State<Pool<MySql>>,
    Query(query_params): Query<QueryRequest>, // 使用Params传参
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
    let response: Response<Page<PhotoModel>> = Response {code:0, message: "success".to_string(), data: Some(page)};

    Ok(Json(response))
}

/**
    上传图片接口
*/
pub async fn upload_photo (
    State(pool): State<Pool<MySql>>,
    mut multipart:Multipart, // 使用form-data传参
) -> Result<Json<Response<()>>, (StatusCode, String)> {
    /*
        这里用 if let 只接受一张图片
        用 while let 接收多张图片
    */
    
    let mut complete = false;

    if let Some(field) = multipart.next_field().await.unwrap() {
        // 文件类型
        let content_type = field.content_type().unwrap_or("").to_string();

        if content_type.starts_with("image/") {
            let name = field.name().unwrap().to_string();

            // 原文件名
            let file_name = field.file_name().unwrap().to_string();

            let file_path = env::var("FILE_PATH").unwrap();
            // 扩展名
            let path = Path::new(&file_name);
            let extension = path.extension().unwrap().to_str().unwrap();

            let uuid = Uuid::new_v4();

            let filename = format!("{}.{}", uuid.to_string(), extension);
            let save_filename = format!("{}/{}", file_path, &filename);

            // 原始数据
            let data = field.bytes().await.unwrap();

            tracing::info!(
                "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
                data.len()
            );

            // 保存上传的文件
            let result = tokio::fs::write(&save_filename, &data)
                .await
                .map_err(|err| err.to_string());

            complete = match result {
                Ok(()) => {
                    let photo = PhotoModel { 
                        id: None, 
                        user_id: Some("no123456".to_string()), 
                        photo_path: Some(filename.to_string()), 
                        remark: Some("this is remark".to_string()), 
                        create_time: None, 
                        update_time: None,
                    };
                    
                    let result = db_photo::insert_or_update_photo(&pool, &photo).await;
                    let b = match result { 
                        Ok(()) => true,
                        Err(_) => false,
                    };
                    b
                },
                Err(_) => {
                    false
                },
            }
        }
        
    } else {
        tracing::info!("Not Found File!");
    }

    let mut code = 4000;
    let mut message = "failure";
    if complete == true {
        code = 0;
        message = "success";
    }

    let response: Response<()> = Response{code:code, message: message.to_string(), data:Some(()) };
    Ok(Json(response))
}

/**
    删除图片接口
*/
pub async fn delete_photos(
    State(pool): State<Pool<MySql>>,
    Json(query_params): Json<QueryCollect<i32>>, // 使用Body传参
) -> Result<Json<Response<()>>, (StatusCode, String)> {

    let rs = match query_params.ids {
        Some(x) => {
            let result = db_photo::delete_photos(&pool, &x).await;
            let b = match result { 
                Ok(()) => true,
                Err(_) => false,
            };
            b     
        },
        None => false,
    };

    let mut code = 4000;
    let mut message = "failure";
    if rs == true {
        code = 0;
        message = "success";
        
    }

    let response: Response<()> = Response{code:code, message: message.to_string(), data:Some(()) };
    Ok(Json(response))
}