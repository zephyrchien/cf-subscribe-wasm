use crate::kv;
use crate::http;
use crate::check;
use crate::types::*;
use crate::error::*;
use futures::future::try_join_all;

pub async fn subscribe(ctx: &Context, form: &Form) -> Result<Response> {
    check!(form, &ctx.passwd, true);

    let proto = form.proto.as_ref().unwrap().as_str();
    let kv = match proto {
        "v2" | "v2ray" => &ctx.kv_v2,
        "ss" | "shadowsocks" => &ctx.kv_ss,
        _ => return Ok(http::not_found()),
    };

    let res = kv::list(kv).await?;

    let text: Vec<String> = try_join_all(
        res.keys
            .into_iter()
            .map(|key| async move { kv::get(kv, key.name).await }),
    )
    .await?;
    Ok(http::new_response(&text.join("\n")))
}
