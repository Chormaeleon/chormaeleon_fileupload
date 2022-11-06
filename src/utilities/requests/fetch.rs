use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use gloo_utils::window;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug)]
pub enum FetchError {
    JsError(JsValue),
    SerdeError(serde_json::error::Error),
    WrongContentType,
    StatusCode(u16),
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

#[allow(dead_code)]
pub async fn get_request_string(url: String) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp = send_request(&request).await?;

    check_status(&resp)?;

    let text = JsFuture::from(resp.text()?).await?;

    match text.as_string() {
        Some(string) => Ok(string),
        None => Err(FetchError::WrongContentType),
    }
}

pub async fn get_request_struct<T: for<'a> serde::de::Deserialize<'a>>(
    url: &str,
) -> Result<T, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);

    let request = Request::new_with_str_and_init(url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    let resp = send_request(&request).await?;

    check_status(&resp)?;

    let result = parse_result(&resp).await?;

    Ok(result)
}

pub async fn post_request_struct<
    PAYLOAD: Serialize,
    RESPONSE: for<'a> serde::de::Deserialize<'a>,
>(
    url: &str,
    payload: PAYLOAD,
) -> Result<RESPONSE, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);

    let serialized = serde_json::to_string(&payload).map_err(FetchError::from)?;
    opts.body(Some(&serialized.into()));

    let request = Request::new_with_str_and_init(url, &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let resp = send_request(&request).await?;

    let result = parse_result(&resp).await?;

    Ok(result)
}

pub async fn delete_request(url: &str) -> Result<(), FetchError> {
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let response = send_request(&request).await?;

    check_status(&response)?;

    Ok(())
}

async fn send_request(request: &Request) -> Result<Response, FetchError> {
    let fetch = window().fetch_with_request(request);

    let resp_value = JsFuture::from(fetch).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let response: Response = resp_value.dyn_into().unwrap();

    Ok(response)
}

async fn parse_result<T: for<'a> serde::de::Deserialize<'a>>(
    response: &Response,
) -> Result<T, FetchError> {
    // Convert this Promise into a rust Future.
    let json = JsFuture::from(response.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let result =
        serde_wasm_bindgen::from_value(json).map_err(|error| FetchError::JsError(error.into()))?;
    Ok(result)
}

fn check_status(response: &Response) -> Result<(), FetchError> {
    let status = response.status();

    match status {
        200 | 201 | 203 | 304 => Ok(()),
        _ => Err(FetchError::StatusCode(status)),
    }
}
