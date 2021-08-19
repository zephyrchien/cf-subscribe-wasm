use serde::{Deserialize, Serialize};

pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen_futures::JsFuture;
pub use js_sys::Promise;
pub use web_sys::{Request, Response, ResponseInit, Url, UrlSearchParams};

// ===== config =====
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub passwd: String,
    pub get_path: String,
    pub put_path: String,
    pub list_path: String,
    pub delete_path: String,
    pub subscribe_path: String,
}

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
    pub tag: Option<String>,
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
            tag: params.get("tag"),
            proto: params.get("proto"),
            passwd: params.get("passwd"),
            token: params.get("token"),
        }
    }
}

// ===== subscribe =====
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
pub struct V2rayConfig {
    #[serde(default = "df_v")] pub v: String,
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    #[serde(default = "df_aid")] pub aid: String,
    #[serde(default = "df_scy")] pub scy: String,
    pub net: String,
    #[serde(default = "df_type")] pub r#type: String,
    #[serde(default)] pub host: String,
    pub path: String,
    pub tls: String,
    #[serde(default)] pub sni: String,
}

fn df_v() -> String { String::from("2") }
fn df_aid() -> String { String::from("1") }
fn df_scy() -> String { String::from("auto") }
fn df_type() -> String { String::from("none") }

#[derive(Serialize, Deserialize)]
pub struct ShadowsocksConfig {
    pub tag: String,
    pub server: String,
    pub server_port: String,
    pub method: String,
    pub password: String,
}
