use std::fmt::Display;

use base64::{
    alphabet,
    engine::general_purpose::{self, GeneralPurpose},
    Engine,
};
use gloo_console::{error, info, warn};
use gloo_dialogs::alert;
use gloo_utils::window;
use serde::Deserialize;
use time::OffsetDateTime;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use yew::prelude::*;

use web_sys::{HtmlDocument, UrlSearchParams};

use crate::service::CONFIG;

const JWT_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD);

const EXPIRES_PAST: &str = "expires=Fri, 31 Dec 1999 23:59:59 GMT;";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Section {
    Soprano1,
    Alto1,
    Tenor1,
    Bass1,
    Soprano2,
    Alto2,
    Tenor2,
    Bass2,
    Conductor,
    Instrument,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Section::Soprano1 => "Sopran 1",
            Section::Alto1 => "Alt 1",
            Section::Tenor1 => "Tenor 1",
            Section::Bass1 => "Bass 1",
            Section::Soprano2 => "Sopran 2",
            Section::Alto2 => "Alt 2",
            Section::Tenor2 => "Tenor 2",
            Section::Bass2 => "Bass 2",
            Section::Conductor => "Dirigent",
            Section::Instrument => "Instrument",
        };
        write!(f, "{}", s)
    }
}

impl ToHtml for Section {
    fn to_html(&self) -> Html {
        html!(self)
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
    let document = gloo_utils::document();

    let doc: HtmlDocument = document.clone().dyn_into().unwrap();

    let param = get_jwt_from_url_param(document);

    if let Ok(Some(p)) = param {
        set_jwt_cookie(&doc, &p);
        if let Err(error) = remove_jwt_from_window_path(&p) {
            warn!("Could not remove the jwt token from the url");
            warn!(error);
        }
        return p;
    }

    if let Ok(Some(jwt)) = get_jwt_from_cookie(doc) {
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
fn get_jwt_from_cookie(doc: HtmlDocument) -> Result<Option<String>, JsValue> {
    Ok(doc
        .cookie()?
        .split("; ")
        .find(|x| x.starts_with("jwt="))
        .map(|x| x.trim_start_matches("jwt=").to_owned()))
}

/// Sets the jwt cookie to the value provided
fn set_jwt_cookie(doc: &HtmlDocument, token: &str) {
    let domain = &CONFIG.get().expect("Config unset").backend_domain;

    unset_jwt_cookie(doc, domain);

    doc.set_cookie(&format!(
        "jwt=Bearer {token}; Domain={domain}; SameSite=Strict; Secure",
    ))
    .unwrap_or_else(show_cookie_set_error);
}

/// Unsets all old variants of the jwt cookie
fn unset_jwt_cookie(doc: &HtmlDocument, domain: &str) {
    doc.set_cookie(&format!("jwt=; {EXPIRES_PAST}"))
        .unwrap_or_else(show_cookie_set_error);
    doc.set_cookie(&format!("jwt=; {EXPIRES_PAST}; Domain={domain}"))
        .unwrap_or_else(show_cookie_set_error);
}

/// Alerts the user about a cookie that could not be set
fn show_cookie_set_error(err: JsValue) {
    error!("Could not set cookie. Error:");
    error!(err);
    alert(
        "Konnte kein Cookie setzen. Sind Cookies für diese Seite eingeschaltet? 
    Cookies sind erforderlich, damit diese Seite funktioniert.",
    )
}

/// Tries to read a JWT from the "token" url parameter.
/// If it succeeds, removes the parameter
fn get_jwt_from_url_param(document: web_sys::Document) -> Result<Option<String>, JsValue> {
    let params = UrlSearchParams::new_with_str(&document.location().unwrap_throw().search()?)?;

    let token = params.get("token");

    Ok(token)
}

/// Changes the window url so that the jwt token string is removed
fn remove_jwt_from_window_path(token: &str) -> Result<(), JsValue> {
    let token_path_param = &format!("token={token}");
    let mut url_without_path_param = window().location().href()?.replace(token_path_param, "");
    if url_without_path_param.contains("?&") {
        url_without_path_param = url_without_path_param.replace("?&", "?");
    } else {
        url_without_path_param = url_without_path_param.replace('?', "");
    }
    window().location().set_href(&url_without_path_param)?;
    Ok(())
}

/// Navigates to the authentication URL set in the configuration
fn redirect_to_login() {
    let auth_url = &CONFIG.get().unwrap().auth_url;
    info!(&format!("Redirecting to {auth_url}"));

    window().location().set_href(auth_url).unwrap_throw();
}
