use crate::types::*;

pub fn forbidden() -> Response {
    let mut init = ResponseInit::new();
    init.status(403);
    init.status_text("Forbidden");
    Response::new_with_opt_str_and_init(None, &init).unwrap()
}

pub fn not_found() -> Response {
    let mut init = ResponseInit::new();
    init.status(404);
    init.status_text("Not Found");
    Response::new_with_opt_str_and_init(None, &init).unwrap()
}

pub fn new_response(message: &str) -> Response {
    let mut init = ResponseInit::new();
    let headers = Headers {
        content_type: "plain/text".to_string(),
    };
    init.status(200);
    init.status_text("OK");
    init.headers(&JsValue::from_serde(&headers).unwrap());
    Response::new_with_opt_str_and_init(Some(message), &init).unwrap()
}
