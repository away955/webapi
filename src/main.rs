#![feature(try_trait_v2)]
#![allow(dead_code)]

use tokio::signal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod apis;
mod auth;
mod entities;
mod middlewares;
mod models;
mod repositories;
mod serializer;
mod services;
mod settings;

#[tokio::main]
async fn main() {
    settings::init();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from(settings::log()))
        .init();

    let app = apis::create_router().await;
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
