use crate::http;
use crate::utils;
use crate::types::*;
use crate::error::*;

pub async fn register(
    ctx: &Context,
    request: &Request,
    sub_path: &str,
) -> Result<Response> {
    let data: Promise = request.json()?;
    let data = JsFuture::from(data).await?;
    match sub_path {
        "/v2ray" => {
            let data: V2rayConfig = data.into_serde()?;
            register_v2ray(ctx, &data).await
        }
        "/shadowsocks" => {
            let data: ShadowsocksConfig = data.into_serde()?;
            register_shadowsocks(ctx, &data).await
        }
        _ => Ok(http::not_found()),
    }
}

async fn register_v2ray(ctx: &Context, data: &V2rayConfig) -> Result<Response> {
    let tag = &data.ps;
    let link =
        format!("vmess://{}", utils::base64(&serde_json::to_string(data)?));
    let _ = ctx
        .kv_v2
        .put(tag.into(), link.into(), JsValue::NULL)
        .await?;
    Ok(http::new_response("registered"))
}

async fn register_shadowsocks(
    ctx: &Context,
    data: &ShadowsocksConfig,
) -> Result<Response> {
    let tag = &data.tag;
    let link = format!(
        "ss://{}@{}:{}#{}",
        utils::base64(&format!("{}:{}", data.method, data.password)),
        data.server,
        data.server_port,
        data.tag
    );
    let _ = ctx
        .kv_ss
        .put(tag.into(), link.into(), JsValue::NULL)
        .await?;
    Ok(http::new_response("registered"))
}
