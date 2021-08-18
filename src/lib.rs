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
) -> Result<Response, JsValue> {
    utils::set_panic_hook();
    let ctx = Context { kv_v2, kv_ss };
    let url: Url = Url::new(&request.url())?;
    let path: String = url.pathname();
    let method: String = request.method();
    if path == "/subscribe" && method == "GET" {
        let form: UrlSearchParams = url.search_params();
        return Ok(sub::subscribe(&ctx, &form).await?);
    }
    if path.starts_with("/register") && method == "POST" {
        let sub_path = &path["/register".len()..];
        return Ok(reg::register(&ctx, &request, sub_path).await?);
    }
    Ok(http::not_found())
}
