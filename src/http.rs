use crate::types::*;

pub fn not_found() -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    init.status(404);
    init.status_text("Not Found");
    Response::new_with_opt_str_and_init(None, &init)
}

pub fn new_response(message: &str) -> Result<Response, JsValue> {
    let mut init = ResponseInit::new();
    let headers = Headers {
        content_type: "plain/text".to_string(),
    };
    init.status(200);
    init.status_text("OK");
    init.headers(&JsValue::from_serde(&headers).map_err(|e| e.to_string())?);
    Response::new_with_opt_str_and_init(Some(message), &init)
}
