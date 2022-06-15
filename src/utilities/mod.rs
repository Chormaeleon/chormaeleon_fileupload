use gloo_utils::document;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlAnchorElement;

pub(crate) mod requests;

pub fn download_from_link(url: &str) {
    let ele: HtmlAnchorElement = document()
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap_throw();
    ele.set_href(url);
    ele.set_download("true");
    ele.click();
    ele.remove();
}
