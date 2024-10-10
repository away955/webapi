#![allow(dead_code)]

use crate::entities::prelude::SysAccount;
use anyhow::{anyhow, Ok, Result};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, IntoActiveModel, QueryFilter};

use crate::entities::sys_account::{self};

pub async fn get_by_id(db: &DbConn, id: i32) -> Result<Option<sys_account::Model>> {
    let data = SysAccount::find_by_id(id).one(db).await?;
    Ok(data)
}

pub async fn get_by_account(db: &DbConn, account: &str) -> Result<Option<sys_account::Model>> {
    let data = SysAccount::find()
        .filter(sys_account::Column::Account.eq(account))
        .one(db)
        .await?;
    Ok(data)
}

pub async fn add(db: &DbConn, model: sys_account::Model) -> Result<i32> {
    let am = model.into_active_model();
    let data = SysAccount::insert(am).exec(db).await.map_err(|err| {
        tracing::warn!("创建账号失败:{}", err.to_string().as_str());
        anyhow!("创建账号失败")
    })?;
    Ok(data.last_insert_id)
}
