pub fn init() {
    dotenvy::dotenv().ok();
}

pub fn db_url() -> String {
    get_value("DB_URL")
}
pub fn host() -> String {
    get_value("Host")
}
pub fn log() -> String {
    get_value("LOG")
}

pub fn jwt_secret() -> String {
    get_value("JWT_SECRET")
}

fn get_value(key: &str) -> String {
    let Ok(value) = dotenvy::var(key) else {
        tracing::error!("{}不存在，请添加配置", key);
        return String::new();
    };
    value
}
