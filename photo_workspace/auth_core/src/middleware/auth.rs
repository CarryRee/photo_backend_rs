use axum::{
    body::{self, BoxBody},
    response::{IntoResponse, Response},
    BoxError, Json,
    http::Request,
};
use std::{
    boxed::Box,
    convert::Infallible,
    task::{Context, Poll},
};
use futures::future::BoxFuture;
use bytes::Bytes;
use tower::{Layer, Service};
use sqlx::pool::Pool;
use tokio::runtime::Runtime;
use common_lib::{
    model::response,
    constant,
};

use crate::{
    model::user_token,
    db::user,
};



#[derive(Clone)]
pub struct AuthLayer {
    pub state: Pool<sqlx::MySql>,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { 
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    state: Pool<sqlx::MySql>,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
    + Clone
    + Send
    + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: axum::body::HttpBody<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {

        let state = self.state.clone();

        tracing::info!("middleware called!");

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let mut authenticate_pass: bool = false;

        if let Some(auth_header) = req.headers().get(constant::AUTHORIZATION) {
            tracing::info!("Parsing authorization header...");
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                    tracing::info!("Parsing token...");
                    let token = auth_str[6..auth_str.len()].trim();

                    if let Ok(token_data) = user_token::UserToken::decode_token(token.to_string())
                    {
                        tracing::info!("Decoding token...");
                        let user_token = token_data.claims;
                        authenticate_pass = true;
                    }
                }
            }
        }
        //这是是拦截信息
        if authenticate_pass {
            Box::pin(async move {
                let res = inner.call(req).await?.map(body::boxed);
                Ok(res)
            })
        } 
        else {
            Box::pin(async move {
                let res: response::Response<()> = response::Response{
                    code: constant::CODE_ACCOUNT_ALREADY_EXISTS, 
                    message: constant::MESSAGE_WRONG_ACCOUNT_OR_PASSWORD.to_string(), 
                    data: None
                };
                Ok(Json(res).into_response())
            })
        }

        
        /* 
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);
        let mut authenticate_pass: bool = false;
        let mut authenticate_username: String = String::from("");

        // Bypass account routes
        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.uri().path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
            if !authenticate_pass {
                if let Some(pool) = req.extensions().get::<Extension<Pool>>() {
                    info!("Connecting to database...");
                    if let Some(auth_header) = req.headers().get(constants::AUTHORIZATION) {
                        info!("Parsing authorization header...");
                        if let Ok(auth_str) = auth_header.to_str() {
                            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                                info!("Parsing token...");
                                let token = auth_str[6..auth_str.len()].trim();
                                if let Ok(token_data) = token_utils::decode_token(token.to_string())
                                {
                                    info!("Decoding token...");
                                    if token_utils::validate_token(&token_data, pool).is_ok() {
                                        info!("Valid token");
                                        authenticate_username = token_data.claims.user_name;
                                        authenticate_pass = true;
                                    } else {
                                        error!("Invalid token");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pass {
            let vals = CasbinVals {
                subject: authenticate_username,
                domain: None,
            };
            req.extensions_mut().insert(vals);
            Box::pin(async move { Ok(inner.call(req).await?.map(body::boxed)) })
        } else {
            Box::pin(async move {
                Ok(Json(ResponseBody::new(
                    constants::MESSAGE_INVALID_TOKEN,
                    constants::EMPTY,
                ))
                .into_response())
            })
        }
        */
    }
}