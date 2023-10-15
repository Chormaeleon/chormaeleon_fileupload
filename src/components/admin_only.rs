use gloo_console::error;
use yew::{function_component, html, Children, Html, Properties};

use super::jwt_context::get_token_data;

#[derive(PartialEq, Properties)]
pub struct AdminOrOwnerProperties {
    pub owner_id: i64,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AdminOrOwner)]
pub fn admin_or_owner(props: &AdminOrOwnerProperties) -> Html {
    if let Ok(user) = get_token_data() {
        if user.is_admin || user.user_id == props.owner_id {
            return html! { { for props.children.iter() } };
        }
    } else {
        error!("Could not get performer data from jwt!");
    }

    html! {}
}

#[derive(PartialEq, Properties)]
pub struct AdminProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AdminOnly)]
pub fn admin_or_owner(props: &AdminProperties) -> Html {
    if let Ok(user) = get_token_data() {
        if user.is_admin {
            return html! { { for props.children.iter() } };
        }
    } else {
        error!("Could not get performer data from jwt!");
    }

    html! {}
}
