use axum::{extract::{State, Query}, Json};
use sqlx::{Pool, MySql};
use axum::http::StatusCode;
use validator::Validate;
use tracing::info;
use auth_core::{
    model::{
        user_token::{UserToken, TokenResp},
        user::SignUser
    },
    db::user,
};
use common_lib::{
    model::response::Response,
    constant,
};

pub async fn sign_in(
    State(pool): State<Pool<MySql>>,
    Json(sign_user): Json<SignUser>, // 使用Json传参
) -> Result<Json<Response<TokenResp>>, (StatusCode, String)> {

    let mut response = Response {
        code: constant::CODE_WRONG_ACCOUNT_OR_PASSWORD, 
        message: constant::MESSAGE_WRONG_ACCOUNT_OR_PASSWORD.to_string(), 
        data:None
    };

    // 判断用户是否存在
    let find_user = user::query_user(&pool, &sign_user.name).await;
    if let Ok(user) = find_user {
        info!("find registered user {:?}.", user);
        
        if user.password.eq(&sign_user.password)  {
            let token = UserToken::generate_token(&user.uuid, &user.name);

            let data = TokenResp {
                token_type: "Bearer".to_string(),
                access_token: token
            };
            response = Response {
                code: constant::CODE_SUCCESS, 
                message: constant::MESSAGE_SUCCESS.to_string(), 
                data:Some(data)
            };
        }
    }
    Ok(Json(response))
}

pub async fn sign_up (
    State(pool): State<Pool<MySql>>,
    Json(sign_user): Json<SignUser>, // 使用Json传参
) -> Result<Json<Response<()>>, (StatusCode, String)> {

    let rs = match sign_user.validate() {
        Ok(_) => true,
        Err(_) => false,
    };
    let mut message = constant::MESSAGE_PARAMETER_ERROR;
    let mut code = constant::CODE_PARAMETER_ERROR;
    
    if rs {
        // 判断用户是否存在
        let find_user = user::query_user(&pool, &sign_user.name).await;
        if let Ok(user) = find_user {
            info!("find registered user {:?}.", user);

            message = constant::MESSAGE_ACCOUNT_ALREADY_EXISTS;
            code = constant::CODE_ACCOUNT_ALREADY_EXISTS;
        }
        else {
            let _ = user::insert_user(&pool, &sign_user).await;

            message = constant::MESSAGE_SUCCESS;
            code = constant::CODE_SUCCESS;
        }
    }

    let response: Response<()> = Response {code:code, message: message.to_string(), data:None};

    Ok(Json(response))
}
