use sea_orm::{Database, DbConn};

use crate::settings;

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}

impl AppState {
    pub async fn new() -> AppState {
        let db = get_db().await;
        Self { db }
    }
}

pub async fn get_db() -> DbConn {
    Database::connect(settings::db_url())
        .await
        .map_err(|err| {
            tracing::error!("数据库连接失败");
            anyhow::anyhow!("数据库连接失败:{}", err.to_string())
        })
        .unwrap()
}
