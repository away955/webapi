mod account;
mod chathub;
mod upload;

use std::{sync::Arc, time::Duration};

use axum::{extract::DefaultBodyLimit, http::Method, middleware, routing::*};
use sea_orm::{Database, DbConn};
use tokio::sync::broadcast::{channel, Sender};
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{auth, models::api_result::ApiResult, settings};

use self::chathub::Payload;

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
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().gzip(true))
        .layer(DefaultBodyLimit::disable())
        .layer(TimeoutLayer::new(Duration::from_secs(3)))
}

/// 需要：jwt授权认证的接口
fn auth_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws", get(chathub::handler))
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

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
    pub chat: Sender<Payload>,
}

impl AppState {
    pub async fn new() -> AppState {
        let db = get_db().await;

        let (tx, _rx) = channel::<Payload>(32);
        Self { db, chat: tx }
    }
}

async fn get_db() -> DbConn {
    Database::connect(settings::db_url())
        .await
        .map_err(|err| anyhow::anyhow!("数据库连接失败:{}", err.to_string()))
        .unwrap()
}
