use axum::{extract::{State, Query}, Json};
use axum::http::StatusCode;
use sqlx::{Pool, MySql};

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