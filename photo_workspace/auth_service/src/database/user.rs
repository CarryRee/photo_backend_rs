
use crate::model::user::{SignUser, User};
use sqlx::{Pool, MySql};
use uuid::Uuid;
use axum::http::StatusCode;

pub async fn insert_user (
    pool: &Pool<MySql>,
    sign_user: &SignUser
) -> Result<(), (StatusCode, String)> {

    tracing::info!("db:insert_user");

    let user = User {
        id: None,
        uuid: Uuid::new_v4().to_string(),
        name: sign_user.name.to_string(),
        password: sign_user.password.to_string(),
        status: 1,
        create_time: None,
        update_time: None,
    };

    let _ = sqlx::query!(
"INSERT INTO user (uuid, name, password, status, update_time)
 VALUES (?, ?, ?, ?, curtime())", user.uuid, user.name, user.password, user.status)
    .fetch_all(pool)
    .await;

    Ok(())
}

pub async fn query_user(
    pool: &Pool<MySql>,
    username: &str
) -> Result<User, (StatusCode, String)> {

    let obj = sqlx::query_as!(User,
"SELECT id, uuid, name, password, status, create_time, update_time FROM user WHERE name = ?", username)
    .fetch_one(pool)
    .await;

    match obj {
        Ok(user) => Ok(user),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}