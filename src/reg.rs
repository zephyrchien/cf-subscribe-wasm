use crate::kv;
use crate::http;
use crate::check;
use crate::utils;
use crate::types::*;
use crate::error::*;

pub async fn register(
    ctx: &Context,
    request: &Request,
    form: &Form,
) -> Result<Response> {
    check!(form, &ctx.passwd, false);

    let data: Promise = request.json()?;
    let data = JsFuture::from(data).await?;

    let (kv, tag, link) = match form.proto.as_ref().unwrap().as_str() {
        "v2" | "/v2ray" => {
            let data: V2rayConfig = data.into_serde()?;
            let link = utils::v2ray_link(&data)?;
            (&ctx.kv_v2, data.ps.to_string(), link)
        }
        "ss" | "/shadowsocks" => {
            let data: ShadowsocksConfig = data.into_serde()?;
            let link = utils::shadowsocks_link(&data)?;
            (&ctx.kv_ss, data.tag.to_string(), link)
        }
        _ => return Ok(http::not_found()),
    };
    kv::put(kv, &tag, link).await?;
    return Ok(http::new_response(&format!("registered: {}\n", &tag)));
}
