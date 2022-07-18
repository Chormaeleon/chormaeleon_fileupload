use gloo_console::error;
use yew::{function_component, html, Children, Properties};

use super::jwt_context::get_token_data;

#[derive(PartialEq, Properties)]
pub struct AdminOrOwnerProperties {
    pub owner_id: i32,
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
