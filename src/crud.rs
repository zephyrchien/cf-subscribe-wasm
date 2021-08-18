use crate::kv;
use crate::http;
use crate::check;
use crate::utils;
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

pub async fn register(
    ctx: &Context,
    request: &Request,
    form: &Form,
) -> Result<Response> {
    check!(form, &ctx.passwd, false);

    let data: Promise = request.json()?;
    let data = JsFuture::from(data).await?;

    let (kv, key, link) = match form.proto.as_ref().unwrap().as_str() {
        "v2" | "v2ray" => {
            let data: V2rayConfig = data.into_serde()?;
            let link = utils::v2ray_link(&data)?;
            let tag = form
                .tag
                .as_ref()
                .map_or_else(|| data.ps.to_string(), |x| x.clone());
            (&ctx.kv_v2, tag, link)
        }
        "ss" | "shadowsocks" => {
            let data: ShadowsocksConfig = data.into_serde()?;
            let link = utils::shadowsocks_link(&data)?;
            let tag = form
                .tag
                .as_ref()
                .map_or_else(|| data.tag.to_string(), |x| x.clone());
            (&ctx.kv_ss, tag, link)
        }
        _ => return Ok(http::not_found()),
    };
    kv::put(kv, &key, link).await?;
    return Ok(http::new_response(&format!("registered: {}\n", &key)));
}

pub async fn fetch(ctx: &Context, form: &Form) -> Result<Response> {
    check!(form, &ctx.passwd, false);
    if form.tag.is_none() {
        return Ok(http::not_found());
    }

    let (kv, key) = match form.proto.as_ref().unwrap().as_str() {
        "v2" | "v2ray" => (&ctx.kv_v2, form.tag.as_ref().unwrap()),
        "ss" | "shadowsocks" => (&ctx.kv_ss, form.tag.as_ref().unwrap()),
        _ => return Ok(http::not_found()),
    };

    let res = kv::get(kv, key).await?;
    return Ok(http::new_response(&res));
}

pub async fn revoke(ctx: &Context, form: &Form) -> Result<Response> {
    check!(form, &ctx.passwd, false);
    if form.tag.is_none() {
        return Ok(http::not_found());
    }

    let (kv, key) = match form.proto.as_ref().unwrap().as_str() {
        "v2" | "v2ray" => (&ctx.kv_v2, form.tag.as_ref().unwrap()),
        "ss" | "shadowsocks" => (&ctx.kv_ss, form.tag.as_ref().unwrap()),
        _ => return Ok(http::not_found()),
    };

    kv::delete(kv, key).await?;
    return Ok(http::new_response(&format!("revoked: {}\n", &key)));
}
