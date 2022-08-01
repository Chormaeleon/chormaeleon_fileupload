use wasm_bindgen::UnwrapThrowExt;
use yew::{function_component, html, Callback, Component, Properties};

use crate::{
    components::{modal::Modal, upload::Upload},
    pages::home::{get_input_text_content, get_selected_value},
    service::material::{
        material_upload_url, update_material, MaterialCategory, MaterialTo, UpdateMaterial,
    },
    utilities::requests::fetch::FetchError,
};

pub const MODAL_MATERIAL_UPDATE: &str = "modalMaterialUpdate";
pub const MODAL_MATERIAL_UPLOAD: &str = "modalMaterialUpload";
const INPUT_UPDATE_MATERIAL_TITLE: &str = "inputUpdateMaterialTitle";
const INPUT_UPDATE_MATERIAL_CATEGORY: &str = "inputUpdateMaterialCategory";

#[derive(Clone, PartialEq, Properties)]
pub struct MaterialUploadModalProperties {
    pub id: i32,
    pub on_success: Callback<String>,
    pub on_error: Callback<String>,
}

#[function_component(MaterialUploadModal)]
pub fn material_upload_modal(props: &MaterialUploadModalProperties) -> Html {
    let props = props.clone();
    html! {
        <Modal
            title={"Übungsmaterial hochladen".to_string() }
            id={ MODAL_MATERIAL_UPLOAD.to_string() }
            actions = { vec![] }
        >
            <form id="formMaterialUpload" class="" name="formUpload" enctype="multipart/form-data">
                <div class="row">
                    <div class="col">
                        <InputMaterialTitle id="inputMaterialTitle" name="title" placeholder_content={PlaceholderOrContent::Placeholder("Titel der Datei".to_string())} form="formMaterialUpload"/>
                    </div>
                    <div class="col">
                        <InputMaterialCategory id="selectMaterialCategory" name="material_category" form="formMaterialUpload"/>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <Upload form_id="formMaterialUpload" field_name="file" target_url={ material_upload_url(props.id) } multiple=false success_callback={ props.on_success } failure_callback={ props.on_error } />
                    </div>
                </div>
            </form>
        </Modal>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MaterialChangeModalProperties {
    pub on_cancel: Callback<()>,
    pub on_success: Callback<MaterialTo>,
    pub on_error: Callback<FetchError>,
    pub material_to_change: Option<MaterialTo>,
}

pub struct MaterialChangeModal {
    sent_request: bool,
}

pub enum Msg {
    Cancel,
    Confirm,
    Success(MaterialTo),
    Error(FetchError),
}

impl Component for MaterialChangeModal {
    type Message = Msg;

    type Properties = MaterialChangeModalProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            sent_request: false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let actions = vec![
            (
                "Abbrechen".to_string(),
                "btn btn-secondary".to_string(),
                ctx.link().callback(|_| Msg::Cancel),
            ),
            (
                "Anpassen".to_string(),
                "btn btn-danger".to_string(),
                ctx.link().callback(|_| Msg::Confirm),
            ),
        ];

        html! {
            <Modal id={MODAL_MATERIAL_UPDATE} title="Übungsmaterial anpassen" actions={ actions }>
                {
                    if let Some(material) = &ctx.props().material_to_change {
                        html!{
                            <div class="row">
                            <div class="col">
                                <InputMaterialTitle id={INPUT_UPDATE_MATERIAL_TITLE} placeholder_content={ PlaceholderOrContent::Content( material.title.clone() ) } form=""/>
                            </div>
                            <div class="col">
                                <InputMaterialCategory id={INPUT_UPDATE_MATERIAL_CATEGORY} selected={ material.category } form=""/>
                            </div>
                            </div>
                        }
                    } else {
                        html!{ { "Nicht zum Update ausgewählt!" } }
                    }
                }
                </Modal>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
            Msg::Confirm => {
                let title = get_input_text_content(INPUT_UPDATE_MATERIAL_TITLE);

                let category = get_selected_value(INPUT_UPDATE_MATERIAL_CATEGORY);
                let category = MaterialCategory::try_from(category.as_str()).unwrap_throw();

                let updated = UpdateMaterial { title, category };

                let id = match &ctx.props().material_to_change {
                    Some(m) => m.id,
                    None => return false,
                };

                ctx.link().send_future(async move {
                    match update_material(id, updated).await {
                        Ok(updated_value) => Msg::Success(updated_value),
                        Err(error) => Msg::Error(error),
                    }
                });

                self.sent_request = true;

                true
            }
            Msg::Success(new_value) => {
                ctx.props().on_success.emit(new_value);
                self.sent_request = false;
                false
            }
            Msg::Error(error) => {
                ctx.props().on_error.emit(error);
                self.sent_request = false;
                false
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum PlaceholderOrContent {
    Placeholder(String),
    Content(String),
}

#[derive(Clone, PartialEq, Properties)]
struct InputMaterialTitleProperties {
    #[prop_or("inputMaterialTitle".to_string())]
    id: String,
    #[prop_or("title".to_string())]
    name: String,
    #[prop_or(PlaceholderOrContent::Placeholder("".to_string()))]
    placeholder_content: PlaceholderOrContent,
    form: String,
}

#[function_component(InputMaterialTitle)]
fn input_material_title(props: &InputMaterialTitleProperties) -> Html {
    let props = props.clone();
    html! {
        <>
        <label for={props.id.clone()}>{ "Name für die Datei" }</label>
        <input
            id={props.id}
            type="text"
            class="form-control"
            name={props.name}
            placeholder={
                if let PlaceholderOrContent::Placeholder(s) = &props.placeholder_content {
                    s.clone()
                } else {
                    "".to_string()
                }
            }
            value={
                if let PlaceholderOrContent::Content(s) = &props.placeholder_content {
                    s.clone()
                } else {
                    "".to_string()
                }
            }
            form={props.form}
        />
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct InputMaterialCategoryProperties {
    #[prop_or("selectMaterialCategory".to_string())]
    id: String,
    #[prop_or("material_category".to_string())]
    name: String,
    form: String,
    #[prop_or(MaterialCategory::Audio)]
    selected: MaterialCategory,
}

#[function_component(InputMaterialCategory)]
fn input_material_category(props: &InputMaterialCategoryProperties) -> Html {
    let props = props.clone();
    html! {
        <>
        <label for={props.id.clone()}>{ "Art der Datei (Playback, ...)" }</label>
        <select id={props.id} name={props.name} class="form-control" form={props.form}>
            <option value="Audio" selected={props.selected == MaterialCategory::Audio}>{ "Audio" }</option>
            <option value="Video" selected={props.selected == MaterialCategory::Video}>{ "Video" }</option>
            <option value="Sheet" selected={props.selected == MaterialCategory::SheetMusic}>{ "Noten" }</option>
            <option value="Other" selected={props.selected == MaterialCategory::Other}>{ "Sonstiges" }</option>
        </select>
        </>
    }
}
