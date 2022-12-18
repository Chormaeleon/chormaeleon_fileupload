use std::fmt::Display;

use base64::{
    alphabet,
    engine::fast_portable::{self, FastPortable},
};
use chrono::Utc;
use gloo_console::error;
use serde::Deserialize;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

use web_sys::{HtmlDocument, UrlSearchParams};

const JWT_ENGINE: FastPortable = FastPortable::from(&alphabet::STANDARD, fast_portable::NO_PAD);

#[derive(Properties, Debug, PartialEq)]
pub struct JWTProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(JWTProvider)]
pub fn jwt_provider(props: &JWTProviderProps) -> Html {
    html! {
        <ContextProvider<String> context={ get_token() }>
            {props.children.clone()}
        </ContextProvider<String>>
    }
}

/// Tries to retrieve the jwt token out of the local storage or a query parameter.
/// If retrieving out of a query parameter, sets it in the local storage.
/// If it cannot be retrieved, sends the user to the endpoint to retrieve a new one.
pub fn get_token() -> String {
    let window = gloo_utils::window();

    let document = gloo_utils::document();

    let doc: HtmlDocument = window.document().unwrap_throw().dyn_into().unwrap();

    let param = get_jwt_from_url_param(document);
    if let Ok(p) = param {
        doc.set_cookie(&format!("jwt=Bearer {p} Domain=localhost"))
            .unwrap_throw();
        return p;
    }

    if let Some(jwt) = doc
        .cookie()
        .unwrap_throw()
        .split("; ")
        .into_iter()
        .find(|x| x.starts_with("jwt="))
        .map(|x| x.trim_start_matches("jwt="))
    {
        return jwt.to_string();
    }

    window
        .location()
        .set_href("http://localhost:8081/turnin")
        .unwrap_throw();

    "".to_string()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Section {
    Soprano,
    Alto,
    Tenor,
    Bass,
    Conductor,
    Instrument,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Section::Soprano => "Sopran",
            Section::Alto => "Alt",
            Section::Tenor => "Tenor",
            Section::Bass => "Bass",
            Section::Conductor => "Dirigent",
            Section::Instrument => "Instrument",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct PerformerData {
    pub section: Section,
    pub user_id: i64,
    pub name: String,
    pub is_admin: bool,
    pub exp: i64,
}

pub fn get_token_data() -> Result<PerformerData, ()> {
    let token = get_token();

    let split_token = token.split('.').collect::<Vec<&str>>();

    let payload = split_token.get(1);

    let payload = match payload {
        Some(p) => p,
        None => {
            error!(format!(
                "JWT could not be split and seems to be invalid: {token}"
            ));
            return Err(());
        }
    };

    let base64_decoded = match base64::decode_engine(payload.as_bytes(), &JWT_ENGINE) {
        Ok(result) => result,
        Err(error) => {
            error!(format!(
                "Error while base64 decoding token: {token}, error: {error}"
            ));
            return Err(());
        }
    };

    let decoded_string = match String::from_utf8(base64_decoded) {
        Ok(string) => string,
        Err(error) => {
            error!(format!(
                "Error while parsing token data to string: {token}, error: {error}"
            ));
            return Err(());
        }
    };

    let data: PerformerData = match serde_json::from_str(&decoded_string) {
        Ok(data) => data,
        Err(error) => {
            error!(format!(
                "Token string could not be parsed:{token}, error: {error}"
            ));
            return Err(());
        }
    };

    if data.exp < Utc::now().timestamp() {
        error!("JWT expired!");
        return Err(());
    }

    Ok(data)
}

fn get_jwt_from_url_param(document: web_sys::Document) -> Result<String, ()> {
    let params =
        UrlSearchParams::new_with_str(&document.location().unwrap_throw().search().unwrap_throw())
            .unwrap_throw();
    let param = match params.get("token") {
        Some(param) => param,
        None => return Err(()),
    };
    Ok(param)
}
