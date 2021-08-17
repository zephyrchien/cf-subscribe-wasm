use crate::http;
use crate::utils;
use crate::types::*;

pub async fn subscribe(
    ctx: &Context,
    form: &UrlSearchParams,
) -> Result<Response, JsValue> {
    type Param = Option<String>;
    let token: Param = form.get("token");
    let proto: Param = form.get("proto");
    if token.is_none() || proto.is_none() {
        return http::not_found();
    }
    let valid_token = utils::md5sum(&utils::month().to_string());
    if token.unwrap() != valid_token {
        return http::not_found();
    }
    let (kv, proto) = match proto.unwrap().as_str() {
        "v2" => (&ctx.kv_v2, "v2ray"),
        "ss" => (&ctx.kv_ss, "shadowsocks"),
        _ => return http::not_found(),
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
    http::new_response(&text.join("\n"))
}
