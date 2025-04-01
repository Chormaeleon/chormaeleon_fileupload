use yew::{function_component, Callback, Children, Html, Properties};

use yew::html;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
#[allow(dead_code)] // modal sizes are static and additional ones may be used later
pub enum ModalSize {
    SM,
    #[default]
    MD,
    LG,
    XL,
    FullScreen,
}

impl ModalSize {
    fn bootstrap_class(&self) -> &str {
        match self {
            ModalSize::SM => "modal-sm",
            ModalSize::MD => "modal-md",
            ModalSize::LG => "modal-lg",
            ModalSize::XL => "modal-xl",
            ModalSize::FullScreen => "modal-fullscreen",
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ModalProperties {
    pub id: String,
    pub title: String,
    #[prop_or_default]
    pub size: ModalSize,
    #[prop_or_default]
    pub children: Children,
    /// The footer action buttons. First String is the title, second the css classes, third the callback to fire onclick.
    #[prop_or_default]
    pub actions: Vec<(String, String, Callback<web_sys::MouseEvent>)>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProperties) -> Html {
    let modal_dialog_class = format!("modal-dialog {}", props.size.bootstrap_class());

    html! {
    <div class="modal fade" id={ props.id.clone() } data-bs-backdrop="static" tabindex="-1" aria-labelledby="createProjectModalLabel" aria-hidden="true">
        <div class={ modal_dialog_class }>
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="createProjectModalLabel">{ &props.title }</h5>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    { for props.children.iter() }
                </div>
                {
                    if !props.actions.is_empty() {
                        html!(
                        <div class="modal-footer">
                        {
                            for props.actions.iter().map(|action| {
                                html!{
                                    <button type="button" class={ &action.1 } data-bs-dismiss="modal" onclick={ &action.2 } >{ &action.0 }</button>
                                }
                            })
                            }
                        </div>)
                    } else {
                        html!()
                    }
                }
            </div>
        </div>
    </div>
    }
}
