use wasm_bindgen::JsValue;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum Error {
    Js(JsValue),
    Serde(SerdeError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self { Error::Js(e) }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self { Error::Serde(e) }
}

impl From<Error> for JsValue {
    fn from(e: Error) -> Self {
        use Error::*;
        match e {
            Js(e) => e,
            Serde(e) => e.to_string().into(),
        }
    }
}
