mod account;
mod upload;
mod ws;

use std::sync::Arc;

use axum::{http::Method, middleware, routing::*};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{auth, middlewares, models::appstate};

pub async fn create_router() -> Router {
    let state = appstate::AppState::new().await;

    Router::new()
        .route("/account/logout", get(account::logout))
        .route("/account/info", get(account::info))
        // 以下为不需要token授权
        .layer(middleware::from_fn(auth::middleware))
        .route("/account/login", post(account::login))
        .route("/account/register", post(account::register))
        .with_state(Arc::new(state))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]),
        )
        .layer(middleware::from_fn(middlewares::logger::middleware))
}
