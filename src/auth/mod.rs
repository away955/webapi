mod secret;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::{extract::Request, middleware::Next, response::IntoResponse};
use axum::{http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::api_result::ApiResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub userid: i32,
}

/// 实现从请求中获取Claims
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiResult<()>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let res = validation_token(parts).await?;
        Ok(res)
    }
}

pub fn create_token(claims: Claims) -> anyhow::Result<String> {
    let keys = &secret::KEYS;
    let token = encode(&Header::default(), &claims, &keys.encoding)?;
    Ok(token)
}

pub async fn validation_token(parts: &mut Parts) -> anyhow::Result<Claims> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| anyhow::anyhow!("access_token 不能为空"))?;

    let keys = &secret::KEYS;
    let access_token = bearer.token();
    let token_data = decode::<Claims>(access_token, &keys.decoding, &Validation::default())
        .map_err(|_| anyhow::anyhow!("access_token 错误"))?;
    Ok(token_data.claims)
}

/// jwt 校验中间件
pub async fn middleware(req: Request, next: Next) -> Result<impl IntoResponse, ApiResult<()>> {
    let (mut parts, body) = req.into_parts();
    validation_token(&mut parts).await?;

    let req = Request::from_parts(parts, body);
    let res = next.run(req).await;
    Ok(res)
}
