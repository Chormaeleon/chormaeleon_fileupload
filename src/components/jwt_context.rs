use std::fmt::Display;

use base64::{
    alphabet,
    engine::general_purpose::{self, GeneralPurpose},
    Engine,
};
use gloo_console::{error, info};
use gloo_utils::window;
use serde::Deserialize;
use time::OffsetDateTime;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

use web_sys::{HtmlDocument, UrlSearchParams};

use crate::service::CONFIG;

const JWT_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD);

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

/// Tries to retrieve the jwt token string out of the local storage or a query parameter that is used to authenticate the user.
/// If retrieving out of a query parameter, sets it in the local storage.
/// If it cannot be retrieved, sends the user to the endpoint to retrieve a new one.
pub fn get_token() -> String {
    let window = gloo_utils::window();

    let document = gloo_utils::document();

    let doc: HtmlDocument = window.document().unwrap_throw().dyn_into().unwrap();

    let param = get_jwt_from_url_param(document);

    if let Ok(p) = param {
        set_jwt_cookie(&doc, &p);
        return p;
    }

    if let Some(jwt) = get_jwt_from_cookie(doc) {
        return jwt;
    }

    redirect_to_login();

    "".to_string()
}

/// Retrieves the token data from the JWT from local storage or URL parameter that is used to authenticate the user.
/// If retrieving out of a query parameter, sets it in the local storage.
/// Redirects to the endpoint configured to get a new token if none was found or it is expired.
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

    let base64_decoded = match JWT_ENGINE.decode(payload.as_bytes()) {
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

    if data.exp < OffsetDateTime::now_local().unwrap().unix_timestamp() {
        error!("JWT expired! Redirecting...");
        redirect_to_login();
        return Err(());
    }

    Ok(data)
}

/// Reads the content of the jwt cookie if it exists
fn get_jwt_from_cookie(doc: HtmlDocument) -> Option<String> {
    doc.cookie()
        .unwrap_throw()
        .split("; ")
        .find(|x| x.starts_with("jwt="))
        .map(|x| x.trim_start_matches("jwt=").to_owned())
}

/// Sets the jwt cookie to the value provided
fn set_jwt_cookie(doc: &HtmlDocument, token: &str) {
    doc.set_cookie(&format!(
        "jwt=Bearer {token}; Domain={}; SameSite=Strict; Secure;",
        CONFIG.get().expect("Config unset").backend_domain
    ))
    .unwrap_throw();
}

/// Tries to read a JWT from the "token" url parameter.
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

/// Navigates to the authentication URL set in the configuration
fn redirect_to_login() {
    let auth_url = &CONFIG.get().unwrap().auth_url;
    info!(&format!("Redirecting to {auth_url}"));

    window().location().set_href(auth_url).unwrap_throw();
}
