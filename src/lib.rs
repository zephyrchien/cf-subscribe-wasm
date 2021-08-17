mod utils;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, ResponseInit, Url, UrlSearchParams};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// kv binding
#[wasm_bindgen]
extern "C" {
    pub type WorkersKv;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn get(
        this: &WorkersKv,
        key: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn put(
        this: &WorkersKv,
        key: JsValue,
        val: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn list(
        this: &WorkersKv,
        prefix: JsValue,
        limit: JsValue,
        cursor: JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Metadata {}

#[derive(Serialize, Deserialize)]
struct GetResult {}

#[derive(Serialize, Deserialize)]
struct ListKey {
    name: String,
    expiration: Option<u64>,
    metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize)]
struct ListResult {
    keys: Vec<ListKey>,
    complete: Option<bool>,
    cursor: Option<String>,
}

struct Context {
    kv_v2: WorkersKv,
    kv_ss: WorkersKv,
}

#[derive(Serialize, Deserialize)]
struct Headers {
    #[serde(rename = "content-type")]
    content_type: String,
}

#[derive(Serialize, Deserialize)]
struct V2rayConfig {
    v: String,
    ps: String,
    add: String,
    port: String,
    id: String,
    aid: String,
    net: String,
    r#type: String,
    host: String,
    path: String,
    tls: String,
}

#[derive(Serialize, Deserialize)]
struct ShadowsocksConfig {
    tag: String,
    server: String,
    server_port: String,
    method: String,
    password: String,
}

fn not_found() -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    init.status(404);
    init.status_text("Not Found");
    Response::new_with_opt_str_and_init(None, &init)
}

fn new_response(message: &str) -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    let headers = Headers {
        content_type: "plain/text".to_string(),
    };
    init.status(200);
    init.status_text("OK");
    init.headers(&JsValue::from_serde(&headers).map_err(|e| e.to_string())?);
    Response::new_with_opt_str_and_init(Some(message), &init)
}

#[wasm_bindgen]
pub async fn handle(
    request: Request,
    kv_v2: WorkersKv,
    kv_ss: WorkersKv,
) -> Result<Response, JsValue> {
    let ctx = Context { kv_v2, kv_ss };
    let url: Url = Url::new(&request.url())?;
    let path: String = url.pathname();
    let method: String = request.method();
    if path == "/subscribe" && method == "GET" {
        let form: UrlSearchParams = url.search_params();
        return subscribe(&ctx, &form).await;
    }
    if path.starts_with("/register") && method == "POST" {
        let sub_path = &path["/register".len()..];
        return register(&ctx, &request, sub_path).await;
    }
    not_found()
}

async fn subscribe(
    ctx: &Context,
    form: &UrlSearchParams,
) -> Result<Response, JsValue> {
    type Param = Option<String>;
    let token: Param = form.get("token");
    let proto: Param = form.get("proto");
    if token.is_none() || proto.is_none() {
        return not_found();
    }
    let valid_token = utils::md5sum(&utils::month().to_string());
    if token.unwrap() != valid_token {
        return not_found();
    }
    let (kv, proto) = match proto.unwrap().as_str() {
        "v2" => (&ctx.kv_v2, "v2ray"),
        "ss" => (&ctx.kv_ss, "shadowsocks"),
        _ => return not_found(),
    };
    let res: JsValue =
        kv.list(JsValue::NULL, JsValue::NULL, JsValue::NULL).await?;
    let res: ListResult = res.into_serde().map_err(|e| e.to_string())?;
    let mut text = Vec::<String>::new();
    for key in res.keys {
        let link: JsValue = kv.get(key.name.into(), JsValue::NULL).await?;
        let link: String = link.into_serde().map_err(|e| e.to_string())?;
        text.push(link)
    }
    new_response(&text.join("\n"))
}

async fn register(
    ctx: &Context,
    request: &Request,
    sub_path: &str,
) -> Result<Response, JsValue> {
    let data = JsFuture::from(request.json()?).await?;
    match sub_path {
        "/v2ray" => {
            let data: V2rayConfig =
                data.into_serde().map_err(|e| e.to_string())?;
            register_v2ray(ctx, &data).await
        }
        "/shadowsocks" => {
            let data: ShadowsocksConfig =
                data.into_serde().map_err(|e| e.to_string())?;
            register_shadowsocks(ctx, &data).await
        }
        _ => not_found(),
    }
}

async fn register_v2ray(
    ctx: &Context,
    data: &V2rayConfig,
) -> Result<Response, JsValue> {
    let tag = &data.ps;
    let link = format!(
        "vmess://{}",
        utils::base64(&serde_json::to_string(data).map_err(|e| e.to_string())?)
    );
    let _ = ctx
        .kv_v2
        .put(tag.into(), link.into(), JsValue::NULL)
        .await?;
    new_response("registered")
}

async fn register_shadowsocks(
    ctx: &Context,
    data: &ShadowsocksConfig,
) -> Result<Response, JsValue> {
    let tag = &data.tag;
    let link = format!(
        "ss://{}@{}:{}#{}",
        utils::base64(&format!("{}:{}", data.method, data.password)),
        data.server,
        data.server_port,
        data.tag
    );
    let _ = ctx
        .kv_ss
        .put(tag.into(), link.into(), JsValue::NULL)
        .await?;
    new_response("registered")
}
