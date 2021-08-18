use crate::types::*;
use crate::error::*;
use crate::WorkersKv;

#[inline]
pub async fn get<T: Into<JsValue>>(kv: &WorkersKv, key: T) -> Result<String> {
    let res: JsValue = kv.get(key.into(), JsValue::NULL).await?;
    let res: String = res.into_serde()?;
    Ok(res)
}

#[inline]
pub async fn put<U, T>(kv: &WorkersKv, key: U, value: T) -> Result<()>
where
    U: Into<JsValue>,
    T: Into<JsValue>,
{
    kv.put(key.into(), value.into(), JsValue::NULL).await?;
    Ok(())
}

#[inline]
pub async fn list(kv: &WorkersKv) -> Result<ListResult> {
    let res: JsValue =
        kv.list(JsValue::NULL, JsValue::NULL, JsValue::NULL).await?;
    let res: ListResult = res.into_serde()?;
    Ok(res)
}
