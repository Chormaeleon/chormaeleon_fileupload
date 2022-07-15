use gloo_console::error;
use yew::{function_component, html, Children, Properties};

use super::jwt_context::get_token_data;

#[derive(PartialEq, Properties)]
pub struct AdminOnlyProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AdminOnly)]
pub fn admin_only(props: &AdminOnlyProperties) -> Html {
    if let Ok(user) = get_token_data() {
        if user.is_admin {
            return html! { { for props.children.iter() } };
        }
    } else {
        error!("Could not get performer data from jwt!");
    }

    html! {}
}
