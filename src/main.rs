#![feature(try_trait_v2)]
#![allow(dead_code)]

mod auth;
mod controllers;
mod entities;
mod middlewares;
mod repositories;
mod serializer;
mod services;
mod settings;

use std::{sync::Arc, time::Duration};

use axum::{extract::DefaultBodyLimit, http::Method, Router};
use controllers::Payload;
use sea_orm::{Database, DbConn};
use tokio::{signal, sync::broadcast::Sender};
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use tokio::sync::broadcast::channel;

use crate::controllers::ApiResult;

#[tokio::main]
async fn main() {
    settings::init();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from(settings::log()))
        .init();

    let state = Arc::new(AppState::new().await);
    let app = Router::new()
        .merge(controllers::routes(state.clone()))
        .fallback(|| async { ApiResult::<()>::err("没有该接口", 404) })
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().gzip(true))
        .layer(DefaultBodyLimit::disable())
        .layer(TimeoutLayer::new(Duration::from_secs(3)));

    let listener = tokio::net::TcpListener::bind(settings::host())
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    tracing::info!("关闭");
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: DbConn,
    pub(crate) chat: Sender<Payload>,
}

impl AppState {
    async fn new() -> AppState {
        let db = get_db().await;

        let (tx, _) = channel::<Payload>(32);
        Self { db, chat: tx }
    }
}

async fn get_db() -> DbConn {
    Database::connect(settings::db_url())
        .await
        .map_err(|err| anyhow::anyhow!("数据库连接失败:{}", err.to_string()))
        .unwrap()
}
