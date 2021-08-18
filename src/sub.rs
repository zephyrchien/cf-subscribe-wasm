use crate::http;
use crate::check;
use crate::types::*;
use crate::error::*;

pub async fn subscribe(ctx: &Context, form: &Form) -> Result<Response> {
    check!(form, &ctx.passwd, true);

    let proto = form.proto.as_ref().unwrap().as_str();
    let kv = match proto {
        "v2" | "v2ray" => &ctx.kv_v2,
        "ss" | "shadowsocks" => &ctx.kv_ss,
        _ => return Ok(http::not_found()),
    };

    let res: JsValue =
        kv.list(JsValue::NULL, JsValue::NULL, JsValue::NULL).await?;
    let res: ListResult = res.into_serde()?;

    let text: Vec<String> =
        futures::future::try_join_all(res.keys.into_iter().map(|key| async {
            let link: JsValue =
                match kv.get(key.name.into(), JsValue::NULL).await {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                };
            let link: String = match link.into_serde() {
                Ok(x) => x,
                Err(e) => return Err(e.to_string().into()),
            };
            Ok(link)
        }))
        .await?;
    Ok(http::new_response(&text.join("\n")))
}
