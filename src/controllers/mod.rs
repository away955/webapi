mod account;
mod chathub;
mod home;
mod result;
mod upload;

pub(crate) use chathub::Payload;
pub(crate) use result::ApiResult;
pub(crate) use result::View;

use std::sync::Arc;

use axum::{middleware, routing::*};

use crate::{auth, AppState};

pub(crate) fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(views_routes())
        .merge(auth_routes(state.clone()))
        .merge(no_auth_routes(state.clone()))
}

/// 需要：jwt授权认证的接口
fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws", get(chathub::handler))
        .route("/account/logout", get(account::logout))
        .route("/account/info", get(account::info))
        .with_state(state)
        .layer(middleware::from_fn(auth::middleware))
}

/// 不需要：jwt授权认证的接口
fn no_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/account/login", post(account::login))
        .route("/account/register", post(account::register))
        .with_state(state)
}

/// 视图
fn views_routes() -> Router {
    Router::new()
        .route("/upload/test", get(upload::view))
        .route("/upload", post(upload::upload))
        .route("/home", get(home::index))
}
