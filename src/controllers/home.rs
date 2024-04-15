use askama::Template;
use axum::response::IntoResponse;

use super::View;

pub(crate) async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        name: "away".to_string(),
    };
    View(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}
