use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub(crate) struct View<T>(pub(crate) T);

impl<T> IntoResponse for View<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("系统错误：{err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "系统错误".to_string())
            }
            .into_response(),
        }
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct ApiResult<T> {
    pub data: Option<T>,
    pub success: bool,
    pub code: i32,
    pub message: Option<String>,
}

impl<T> IntoResponse for ApiResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T> ApiResult<T>
where
    T: Serialize,
{
    pub fn new(data: Option<T>, code: i32, success: bool, message: Option<String>) -> ApiResult<T> {
        Self {
            code,
            success,
            message,
            data,
        }
    }
    pub fn ok_none() -> ApiResult<T> {
        ApiResult::<T>::new(None, 0, true, None)
    }

    pub fn ok(data: T) -> ApiResult<T> {
        ApiResult::<T>::new(Some(data), 0, true, None)
    }

    pub fn err(message: &str, code: i32) -> ApiResult<T> {
        ApiResult::<T>::new(None, code, false, Some(message.to_string()))
    }
}

impl<T> From<anyhow::Error> for ApiResult<T>
where
    T: Serialize,
{
    fn from(value: anyhow::Error) -> Self {
        ApiResult::err(value.to_string().as_str(), -1)
    }
}

impl<T> From<anyhow::Result<T>> for ApiResult<T>
where
    T: Serialize,
{
    fn from(value: anyhow::Result<T>) -> Self {
        match value {
            Ok(data) => ApiResult::ok(data),
            Err(err) => err.into(),
        }
    }
}
// 当前版本(ightly-x86_64-pc-windows-msvc)编译失败
// #![feature(try_trait_v2)]
// use std::{convert::Infallible, ops::FromResidual};
// impl<T> FromResidual<anyhow::Result<Infallible>> for ApiResult<T>
// where
//     T: Serialize,
// {
//     fn from_residual(residual: anyhow::Result<Infallible>) -> Self {
//         match residual {
//             Ok(_) => ApiResult::new(None, 0, true, None),
//             Err(err) => err.into(),
//         }
//     }
// }

// impl<T> FromResidual<anyhow::Error> for ApiResult<T>
// where
//     T: Serialize,
// {
//     fn from_residual(residual: anyhow::Error) -> Self {
//         residual.into()
//     }
// }
