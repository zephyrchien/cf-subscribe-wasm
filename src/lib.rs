mod kv;
mod http;
mod crud;
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
    pub async fn delete(
        this: &WorkersKv,
        key: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn list(
        this: &WorkersKv,
        prefix: JsValue,
        limit: JsValue,
        cursor: JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[rustfmt::skip]
#[wasm_bindgen]
pub async fn handle(
    request: Request,
    kv_v2: WorkersKv,
    kv_ss: WorkersKv,
    config: JsValue,
) -> Result<Response, JsValue> {
    utils::set_panic_hook();

    let Config { 
        passwd, 
        ref get_path, ref put_path, ref list_path,
        ref delete_path, ref subscribe_path
    } = config.into_serde().map_err(|e| e.to_string())?;

    let ctx = Context {
        kv_v2,
        kv_ss,
        passwd,
    };
    
    let url: Url = Url::new(&request.url())?;
    let path: String = url.pathname();
    let form: Form = url.search_params().into();
    let method: String = request.method();

    if path == *subscribe_path && method == "GET"{
        return Ok(crud::subscribe(&ctx, &form).await?);
    }

    if path == *get_path && method == "GET" {
        return Ok(crud::fetch(&ctx, &form).await?)
    }
    
    if path == *put_path && method == "POST" {
        return Ok(crud::register(&ctx, &request, &form).await?)
    }

    if path == *list_path && method == "GET" {
        return Ok(crud::list(&ctx, &form).await?)
    }

    if path == *delete_path && method == "GET" {
        return Ok(crud::revoke(&ctx, &form).await?)
    }

    Ok(http::not_found())
}
