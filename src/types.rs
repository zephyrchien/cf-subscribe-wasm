use serde::{Deserialize, Serialize};

pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen_futures::JsFuture;
pub use js_sys::Promise;
pub use web_sys::{Request, Response, ResponseInit, Url, UrlSearchParams};

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

pub struct Context {
    pub kv_v2: crate::WorkersKv,
    pub kv_ss: crate::WorkersKv,
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "content-type")]
    pub content_type: String,
}

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
