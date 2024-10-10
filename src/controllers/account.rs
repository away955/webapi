use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    auth::Claims,
    services::account::{self, *},
};

use super::{ApiResult, AppState};


/// 登录
#[utoipa::path(
        post,
        path="/account/login",
        request_body=LoginDTO,
        tag="账号管理"
    )]
pub(super) async fn login(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginDTO>,
) -> ApiResult<LoginModel> {
    account::login(&state.db, &dto).await.into()
}

/// 注册
#[utoipa::path(    
    post,    
    path="/account/register",    
    request_body=LoginDTO,    
    tag="账号管理" 
  )]
pub(super) async fn register(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginDTO>,
) -> ApiResult<i32> {
    account::register(&state.db, &dto).await.into()
}

/// 个人信息
#[utoipa::path(get, path = "/account/info", tag = "账号管理",security(("Authorization" = [])))]
pub(super) async fn info(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> ApiResult<InfoModel> {
    account::info(&state.db, claims.userid).await.into()
}

/// 登出
#[utoipa::path(get, path = "/account/logout", tag = "账号管理",security(("Authorization" = [])))]
pub(super) async fn logout() -> ApiResult<()> {
    ApiResult::ok_none()
}
