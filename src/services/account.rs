use anyhow::{anyhow, Context, Ok, Result};
use chrono::{Duration, Local, Utc};
use sea_orm::prelude::DateTime;
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth,
    entities::sys_account,
    repositories::account::{self, get_by_id},
    serializer::datetime_format,
};

#[derive(Debug, Serialize)]
pub struct LoginModel {
    pub access_token: String,
    pub token_type: String,
    pub token_exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct InfoModel {
    pub account: String,
    pub email: String,
    #[serde(with = "datetime_format")]
    pub created: DateTime,
}

pub async fn login(db: &DbConn, dto: &LoginDTO) -> Result<LoginModel> {
    let user = account::get_by_account(db, &dto.account)
        .await?
        .with_context(|| anyhow!("账号不存在:{}", &dto.account))?;

    let sign = crypto_password(&dto.password, &user.salt);
    if user.signature.ne(&sign) {
        return Err(anyhow!("密码错误"));
    }

    let local_time = Local::now() + Duration::days(10);
    let utc_time = local_time.with_timezone(&Utc);

    let token = auth::create_token(auth::Claims {
        sub: "away".to_string(),
        userid: user.id,
        exp: utc_time.timestamp(),
    })?;

    let data = LoginModel {
        access_token: token,
        token_type: "Bearer".to_string(),
        token_exp: utc_time.timestamp(),
    };
    Ok(data)
}

pub async fn register(db: &DbConn, dto: &LoginDTO) -> Result<i32> {
    let uuid = Uuid::new_v4().to_string();
    let sign = crypto_password(&dto.password, &uuid);
    let model = sys_account::Model {
        id: 0,
        account: dto.account.to_string(),
        signature: sign,
        salt: uuid,
        email: "mrjun@qq.com".to_string(),
        created: Local::now().naive_local(),
    };
    let data = account::add(db, model).await?;
    Ok(data)
}

pub async fn info(db: &DbConn, id: i32) -> Result<InfoModel> {
    let user = get_by_id(db, id)
        .await?
        .with_context(|| anyhow!("账号不存在"))?;

    let data = InfoModel {
        account: user.account,
        email: user.email,
        created: user.created,
    };
    Ok(data)
}

fn crypto_password(password: &str, salt: &str) -> String {
    let digest = md5::compute(format!("{}_{}", password, salt));
    format!("{:x}", digest)
}
