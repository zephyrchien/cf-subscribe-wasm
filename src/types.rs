use serde::{Deserialize, Serialize};

pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen_futures::JsFuture;
pub use js_sys::Promise;
pub use web_sys::{Request, Response, ResponseInit, Url, UrlSearchParams};

// ===== ctx =====
pub struct Context {
    pub kv_v2: crate::WorkersKv,
    pub kv_ss: crate::WorkersKv,
    pub passwd: String,
}

// ===== kv binding =====
#[derive(Serialize, Deserialize)]
pub struct Metadata {}

#[derive(Serialize, Deserialize)]
pub struct GetResult {}

#[derive(Serialize, Deserialize)]
pub struct ListKey {
    pub name: String,
    pub expiration: Option<u64>,
    pub metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize)]
pub struct ListResult {
    pub keys: Vec<ListKey>,
    pub complete: Option<bool>,
    pub cursor: Option<String>,
}

// ===== http =====
#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "content-type")]
    pub content_type: String,
}

pub struct Form {
    pub proto: Option<String>,
    pub passwd: Option<String>,
    pub token: Option<String>,
}

impl Form {
    #[rustfmt::skip]
    #[inline]
    pub fn auth(&self, passwd: &str, allow_token: bool) -> bool {
        use crate::utils::month;
        use crate::utils::md5sum;
        self.passwd.as_ref().map_or(false, |x| x == passwd)
        || (allow_token && self.token.as_ref().map_or(false, |x|
            *x == md5sum(&month().to_string())
        ))
    }
}

impl From<UrlSearchParams> for Form {
    fn from(params: UrlSearchParams) -> Self {
        Form {
            proto: params.get("proto"),
            passwd: params.get("passwd"),
            token: params.get("token"),
        }
    }
}

// ===== subscribe =====
#[derive(Serialize, Deserialize)]
pub struct V2rayConfig {
    pub v: String,
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    pub aid: String,
    pub net: String,
    pub r#type: String,
    pub host: String,
    pub path: String,
    pub tls: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShadowsocksConfig {
    pub tag: String,
    pub server: String,
    pub server_port: String,
    pub method: String,
    pub password: String,
}
