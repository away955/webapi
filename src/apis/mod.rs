mod account;
mod upload;
mod ws;

use std::{sync::Arc, time::Duration};

use axum::{extract::DefaultBodyLimit, http::Method, middleware, routing::*};
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    auth,
    models::{api_result::ApiResult, appstate::AppState},
};

pub async fn create_router() -> Router {
    let state = Arc::new(AppState::new().await);

    Router::new()
        .merge(auth_router(state.clone()))
        .merge(no_auth_router(state.clone()))
        .fallback(|| async { ApiResult::<()>::err("没有该接口", 404) })
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]),
        )
        .layer(CompressionLayer::new().gzip(true))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(3)))
        .layer(DefaultBodyLimit::disable())
}

/// 需要：jwt授权认证的接口
fn auth_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/account/logout", get(account::logout))
        .route("/account/info", get(account::info))
        .with_state(state)
        .layer(middleware::from_fn(auth::middleware))
}

/// 不需要：jwt授权认证的接口
fn no_auth_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/account/login", post(account::login))
        .route("/account/register", post(account::register))
        .with_state(state)
}
