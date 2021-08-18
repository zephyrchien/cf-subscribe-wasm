mod kv;
mod sub;
mod reg;
mod http;
mod types;
mod error;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use types::*;

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

#[wasm_bindgen]
pub async fn handle(
    request: Request,
    kv_v2: WorkersKv,
    kv_ss: WorkersKv,
    passwd: String,
) -> Result<Response, JsValue> {
    utils::set_panic_hook();
    let ctx = Context {
        kv_v2,
        kv_ss,
        passwd,
    };
    let url: Url = Url::new(&request.url())?;
    let path: String = url.pathname();
    let method: String = request.method();
    let form: Form = url.search_params().into();

    if path == "/subscribe" && method == "GET" {
        return Ok(sub::subscribe(&ctx, &form).await?);
    }
    if path == "/register" && method == "POST" {
        return Ok(reg::register(&ctx, &request, &form).await?);
    }
    Ok(http::not_found())
}
