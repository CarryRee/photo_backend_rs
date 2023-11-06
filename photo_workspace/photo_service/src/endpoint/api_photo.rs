use axum::{extract::{State, Query}, Json};
use axum::http::StatusCode;
use sqlx::{Pool, MySql};

use crate::model::photo_model::{PhotoModel, QueryRequest};
use crate::database::db_photo;
use common_lib::model::response::Response;

pub async fn get_photos(
    State(pool): State<Pool<MySql>>,
    Query(Query_params): Query<QueryRequest>,
) -> Result<Json<Response<Vec<PhotoModel>>>, (StatusCode, String)> {

    // 开启事务

    let photo_models = db_photo::query_photos(&pool, 1, 10).await?;
    let response = Response {code:0, message: "success".to_string(), data: photo_models};

    Ok(Json(response))
}