use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

use crate::settings;

pub(crate) static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = settings::jwt_secret();
    Keys::new(secret)
});

pub(crate) struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: String) -> Self {
        let key = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(key),
            decoding: DecodingKey::from_secret(key),
        }
    }
}
