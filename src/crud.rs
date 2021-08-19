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

    let list = kv::list(kv).await?;

    let text = match proto {
        "v2" | "v2ray" => {
            try_join_all(list.keys.into_iter().map(|key| async move {
                let data = kv::get(kv, key.name).await?;
                let data: V2rayConfig = serde_json::from_str(&data)?;
                utils::v2ray_link(&data)
            }))
            .await?
        }
        "ss" | "shadowsocks" => {
            try_join_all(list.keys.into_iter().map(|key| async move {
                let data = kv::get(kv, key.name).await?;
                let data: ShadowsocksConfig = serde_json::from_str(&data)?;
                utils::shadowsocks_link(&data)
            }))
            .await?
        }
        _ => return Ok(http::not_found()),
    };

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

    let (kv, key, payload) = match form.proto.as_ref().unwrap().as_str() {
        "v2" | "v2ray" => {
            let data: V2rayConfig = data.into_serde()?;
            let payload = serde_json::to_string(&data)?;
            let tag = form
                .tag
                .as_ref()
                .map_or_else(|| data.ps.to_string(), |x| x.clone());
            (&ctx.kv_v2, tag, payload)
        }
        "ss" | "shadowsocks" => {
            let data: ShadowsocksConfig = data.into_serde()?;
            let payload = serde_json::to_string(&data)?;
            let tag = form
                .tag
                .as_ref()
                .map_or_else(|| data.tag.to_string(), |x| x.clone());
            (&ctx.kv_ss, tag, payload)
        }
        _ => return Ok(http::not_found()),
    };
    kv::put(kv, &key, payload).await?;
    return Ok(http::new_response(&format!("registered: {}\n", &key)));
}

pub async fn fetch(ctx: &Context, form: &Form) -> Result<Response> {
    check!(form, &ctx.passwd, false);
    if form.tag.is_none() {
        return Ok(http::not_found());
    }

    let key = form.tag.as_ref().unwrap();
    let link = match form.proto.as_ref().unwrap().as_str() {
        "v2" | "v2ray" => {
            let data = kv::get(&ctx.kv_v2, key).await?;
            let data: V2rayConfig = serde_json::from_str(&data)?;
            utils::v2ray_link(&data)?
        }
        "ss" | "shadowsocks" => {
            let data = kv::get(&ctx.kv_ss, key).await?;
            let data: ShadowsocksConfig = serde_json::from_str(&data)?;
            utils::shadowsocks_link(&data)?
        }
        _ => return Ok(http::not_found()),
    };

    Ok(http::new_response(&link))
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

pub async fn list(ctx: &Context, form: &Form) -> Result<Response> {
    check!(form, &ctx.passwd, true);

    let proto = form.proto.as_ref().unwrap().as_str();
    let kv = match proto {
        "v2" | "v2ray" => &ctx.kv_v2,
        "ss" | "shadowsocks" => &ctx.kv_ss,
        _ => return Ok(http::not_found()),
    };

    let list = kv::list(kv).await?;
    let keys: Vec<String> = list.keys.into_iter().map(|key| key.name).collect();
    Ok(http::new_response(&format!("tags:\n{}\n", keys.join(", "))))
}
