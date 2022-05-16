use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use web_sys::UrlSearchParams;

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

    let storage = window.local_storage().unwrap_throw().unwrap_throw();

    match storage.get_item("jwt").unwrap_throw() {
        Some(jwt) => return jwt,
        None => (),
    }

    let document = window.document().unwrap_throw();

    let param = match get_jwt_from_url_param(document) {
        Ok(value) => value,
        Err(_) => {
            window
                .location()
                .set_href("http://localhost:8081/turnin")
                .unwrap_throw();
            return "".to_string();
        }
    };

    storage.set_item("jwt", &param).unwrap_throw();

    param
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
