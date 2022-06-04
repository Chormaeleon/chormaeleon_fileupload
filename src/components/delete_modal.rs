use web_sys::MouseEvent;
use yew::{function_component, Callback, Properties, Children};

use yew::html;

use crate::components::modal::Modal;

#[derive(PartialEq, Properties)]
pub struct DeleteModalProperties {
    pub id: String,
    pub title: String,
    pub children: Children,
    /// The footer action buttons. First String is the title, second the css classes, third the callback to fire onclick.
    pub on_cancel: Callback<MouseEvent>,
    pub on_confirm: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
pub fn modal(props: &DeleteModalProperties) -> Html {
    let actions = vec![
        (
            "Abbrechen".to_string(),
            "btn btn-secondary".to_string(),
            props.on_cancel.clone(),
        ),
        (
            "Löschen".to_string(),
            "btn btn-danger".to_string(),
            props.on_confirm.clone(),
        ),
    ];

    html! {
        <Modal id={props.id.clone()} title={props.title.clone()} actions={ actions }>
            { for props.children.iter() }
        </Modal>
    }
}
