

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Request,
    extract::State,
    Json,
    middleware::Next,
};
use sqlx::{Pool, MySql};

use crate::{
    model::user_token,
    db::user,
};

use common_lib::{
    model::response,
    constant,
};


pub async fn auth(State(pool): State<Pool<MySql>>, mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(constant::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    tracing::info!("middle called");

    let mut uuid: Option<String> = None;

    let err: response::Response<()> = response::Response{
        code: constant::CODE_ACCOUNT_ALREADY_EXISTS, 
        message: constant::MESSAGE_WRONG_ACCOUNT_OR_PASSWORD.to_string(), 
        data: None
    };

    if let Some(auth_str) = auth_header {

        if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
            tracing::info!("Parsing token...");
            let token = auth_str[6..auth_str.len()].trim();

            if let Ok(token_data) = user_token::UserToken::decode_token(token.to_string())  {
                tracing::info!("Decoding token...");
                let user_token: user_token::UserToken = token_data.claims;

                if let Ok(user) = user::query_user_by_uuid(&pool, &user_token.name, &user_token.uuid).await {
                    uuid = Some(user.uuid);
                } 
            }
        }
    } else {
        //return Err(StatusCode::UNAUTHORIZED);
        return Ok(Json(err).into_response());
    };
    
   let result = match uuid {
        Some(uuid) => {
            req.extensions_mut().insert(uuid);
            Ok(next.run(req).await)
        },
        None => {
            //Err(StatusCode::UNAUTHORIZED)
            Ok(Json(err).into_response())
        }
    };
    result
}