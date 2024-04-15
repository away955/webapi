use anyhow::anyhow;
use askama::Template;
use axum::{extract::Multipart, response::IntoResponse};

use crate::controllers::ApiResult;

use super::View;

pub(super) async fn upload(mut multipart: Multipart) -> ApiResult<()> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| anyhow!("上传文件失败：{err}"))?
    {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        tracing::info!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }

    ApiResult::ok_none()
}

pub(super) async fn view() -> impl IntoResponse {
    View(UpdateTemplate {})
}

#[derive(Template)]
#[template(path = "upload.html")]
struct UpdateTemplate;
