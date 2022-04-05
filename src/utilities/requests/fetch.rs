use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug)]
pub enum FetchError {
    JsError(JsValue),
    SerdeError(serde_json::error::Error),
    WrongContentType,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self::JsError(value)
    }
}

impl From<serde_json::error::Error> for FetchError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::SerdeError(error)
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub async fn get_request_string(url: String) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;

    match text.as_string() {
        Some(string) => Ok(string),
        None => Err(FetchError::WrongContentType),
    }
}

pub async fn get_request_struct<T: for<'a> serde::de::Deserialize<'a>>(
    url: String,
) -> Result<T, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let result: T = json.into_serde()?;

    Ok(result)
}
