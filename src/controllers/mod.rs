mod account;
mod chathub;
mod home;
mod result;
mod upload;

pub(crate) use chathub::Payload;
pub(crate) use result::ApiResult;
pub(crate) use result::View;

use axum::{middleware, routing::*};
use std::sync::Arc;
use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::services;
use crate::{auth, AppState};

/// 配置swagger文档
#[derive(OpenApi)]
#[openapi(
    paths(account::login, account::register, account::info, account::logout),
    components(schemas(services::account::LoginDTO)),  
    info(
        title = "性能屌炸天的后台接口",
        description = "后台管理接口文档",
        version = "1.0.0"
    ),
    modifiers(&SecurityAddon),     
)]
struct Swagger;
struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

pub(crate) fn routes(state: Arc<AppState>) -> Router { 
    Router::new()
        .merge(views_routes())
        .merge(auth_routes(state.clone()))
        .merge(no_auth_routes(state.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", Swagger::openapi()))         
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


