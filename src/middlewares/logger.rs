use axum::{
    body::{Body, Bytes},
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

use crate::controllers::ApiResult;

/// http请求日志中间件
pub(crate) async fn middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiResult<()>> {
    let (parts, body) = req.into_parts();
    let host = format!("{} {}", parts.method, parts.uri);
    let request = format!("request: {}", host);
    let bytes = buffer_and_print(&request, body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let response = format!("response: {}", host);
    let bytes = buffer_and_print(&response, body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, ApiResult<()>>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            let msg = format!("failed to read {direction} body: {err}");
            let body = ApiResult::err(msg.as_str(), -1);
            return Err(body);
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("\n{direction}\n{body:?}");
    }

    Ok(bytes)
}
