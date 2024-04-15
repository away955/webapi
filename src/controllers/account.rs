use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    auth::Claims,
    services::account::{self, *},
};

use super::{ApiResult, AppState};

pub(super) async fn login(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginDTO>,
) -> ApiResult<LoginModel> {
    account::login(&state.db, &dto).await.into()
}

pub(super) async fn register(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginDTO>,
) -> ApiResult<i32> {
    account::register(&state.db, &dto).await.into()
}

pub(super) async fn info(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> ApiResult<InfoModel> {
    account::info(&state.db, claims.userid).await.into()
}

pub(super) async fn logout() -> ApiResult<()> {
    ApiResult::ok_none()
}
