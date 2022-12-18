mod audio;
mod material_modals;
mod other_and_all;
mod sheet;
mod video;

use gloo_console::{error, warn};
use gloo_dialogs::alert;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Component, Html, Properties};

use crate::{
    components::{
        admin_only::AdminOrOwner,
        delete_modal::DeleteModal,
        material::material_modals::{
            MaterialChangeModal, MaterialUploadModal, MODAL_MATERIAL_UPDATE, MODAL_MATERIAL_UPLOAD,
        },
    },
    service::material::{delete_material, material_by_project, MaterialCategory, MaterialTo},
    utilities::requests::fetch::FetchError,
};

use self::{
    audio::audio_list, other_and_all::other_and_all_list, sheet::sheet_list, video::video_list,
};

const MODAL_MATERIAL_DELETE: &str = "modalMaterialDelete";

pub struct Material {
    pub material: Vec<MaterialTo>,
    change_selected_material: Option<MaterialTo>,
    delete_selected_material: Option<MaterialTo>,
}

pub enum Msg {
    MaterialUploadSuccess(String),
    MaterialUploadError(String),
    MaterialFetchSuccess(Vec<MaterialTo>),
    MaterialFetchError(FetchError),
    Update(UpdateMessage),
    Delete(DeleteMessage),
}

pub enum UpdateMessage {
    ButtonClick(MaterialTo),
    Success(MaterialTo),
    Error(FetchError),
    Cancel,
}

pub enum DeleteMessage {
    DeleteButtonClick(MaterialTo),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success,
    Fail(FetchError),
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct MaterialProperties {
    pub id: i64,
    pub project_owner: i64,
}

impl Component for Material {
    type Message = Msg;

    type Properties = MaterialProperties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let id = ctx.props().id;
        ctx.link().send_future(async move {
            match material_by_project(id).await {
                Ok(material) => Msg::MaterialFetchSuccess(material),
                Err(error) => Msg::MaterialFetchError(error),
            }
        });

        Self {
            material: Vec::new(),
            change_selected_material: None,
            delete_selected_material: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let material = &self.material;
        html! {
            <>
            <AdminOrOwner owner_id={ ctx.props().project_owner }>
                <div class="row mt-2">
                    <div class="col text-end">
                        <button type="button" class="btn btn-danger" data-bs-toggle="modal" data-bs-target={format!("#{MODAL_MATERIAL_UPLOAD}")}>
                            { "Übungsmaterial hinzufügen" }
                        </button>
                    </div>
                </div>
            </AdminOrOwner>

            {
                audio_list(ctx.props().id, material.iter().filter(|x| x.category == MaterialCategory::Audio).collect())
            }
            <hr class="bg-secondary border-1 border-top border-secondary"/>
            {
                video_list(ctx, material.iter().filter(|x| x.category == MaterialCategory::Video).collect())
            }
            <hr class="bg-secondary border-1 border-top border-secondary"/>
            {
                sheet_list(ctx.props().id, material.iter().filter(|x| x.category == MaterialCategory::SheetMusic).collect())
            }
            <hr class="bg-secondary border-1 border-top border-secondary"/>
            {
                other_and_all_list(ctx, material.iter().collect())
            }

            <AdminOrOwner owner_id={ ctx.props().project_owner }>
                <MaterialUploadModal
                    id={ctx.props().id}
                    on_success={ctx.link().callback(Msg::MaterialUploadSuccess)}
                    on_error={ctx.link().callback(Msg::MaterialUploadError)}
                />

                <MaterialChangeModal
                    on_cancel={ctx.link().callback(|_| Msg::Update(UpdateMessage::Cancel))}
                    on_error={ctx.link().callback(|error| Msg::Update(UpdateMessage::Error(error)))}
                    on_success={ctx.link().callback(|updated_material| Msg::Update(UpdateMessage::Success(updated_material)))}
                    material_to_change={self.change_selected_material.clone()}
                />

                <DeleteModal id={MODAL_MATERIAL_DELETE}
                    title="Material wirklich löschen?"
                    on_cancel={ ctx.link().callback(|e| Msg::Delete(DeleteMessage::AbortClick(e))) }
                    on_confirm={ ctx.link().callback(|e| Msg::Delete(DeleteMessage::AcceptClick(e)))  }
                >
                    if let Some(mat) = &self.delete_selected_material {
                        <p> { "Beschreibung: " } { &mat.title } </p>
                        <p> { "Dateiname: " } <i> { &mat.file_name } </i> </p>
                    } else {
                        { "Kein zu löschendes Element ausgewählt!" }
                    }
                </DeleteModal>
            </AdminOrOwner>
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MaterialUploadSuccess(text) => {
                let entry: MaterialTo = serde_json::from_str(&text).unwrap_throw();
                self.material.push(entry);
                true
            }
            Msg::MaterialUploadError(text) => {
                alert("Ein Fehler ist aufgetreten. Bitte versuche es erneut und wende dich dann an den/die Administrator*in");
                error!(format!(
                    "got non successful status from metadata upload. Error text: {}",
                    text
                ));
                true
            }
            Msg::Delete(message) => match message {
                DeleteMessage::DeleteButtonClick(item) => {
                    self.delete_selected_material = Some(item);
                    true
                }
                DeleteMessage::AcceptClick(_) => {
                    let material_id = match &self.delete_selected_material {
                        Some(m) => m.id,
                        None => {
                            error!("Clicked accept without selected material to delete!");
                            return false;
                        }
                    };
                    ctx.link().send_future(async move {
                        match delete_material(material_id).await {
                            Ok(_) => Msg::Delete(DeleteMessage::Success),
                            Err(error) => Msg::Delete(DeleteMessage::Fail(error)),
                        }
                    });
                    false
                }
                DeleteMessage::AbortClick(_) => {
                    self.delete_selected_material = None;
                    true
                }
                DeleteMessage::Success => {
                    let item = match self.delete_selected_material.take() {
                        Some(content) => content,
                        None => {
                            error!("Succesfully deleted material, but no item was found!");
                            return true;
                        }
                    };

                    self.material.retain(|entry| entry.id != item.id);

                    true
                }
                DeleteMessage::Fail(error) => {
                    match error {
                        FetchError::StatusCode(code) => {
                            if code == 404 {
                                alert("Die Datei wurde bereits gelöscht!");
                                warn!("Could not delete material, got 404.");
                                return false;
                            }
                            alert(&format!("Could not delete material, got status {code}"));
                            warn!(format!(
                                "Konnte die Datei nicht löschen, Statuscode {}",
                                code
                            ))
                        }
                        _ => warn!("Could not delete material!"),
                    }
                    alert("Die Datei konnte nicht gelöscht werden! Details siehe Konsole.");
                    warn!(format!("{:?}", error));
                    false
                }
            },
            Msg::MaterialFetchSuccess(material) => {
                self.material = material;
                sort_material(&mut self.material);
                true
            }
            Msg::MaterialFetchError(error) => {
                error!(format!("Could not fetch material! Error: {}", error));
                alert("Fehler: Konnte das Material nicht laden. Überprüfe Deine Internetverbindung, lade die Seite neu und wende Dich ansonsten an den/die Administrator*in.");
                false
            }
            Msg::Update(message) => match message {
                UpdateMessage::ButtonClick(material) => {
                    self.change_selected_material = Some(material);
                    true
                }
                UpdateMessage::Success(new_material) => {
                    self.material.retain(|m| {
                        m.id != self.change_selected_material.as_ref().unwrap_throw().id
                    });
                    self.material.push(new_material);
                    sort_material(&mut self.material);
                    self.change_selected_material = None;
                    true
                }
                UpdateMessage::Error(error) => {
                    alert("Konnte die neuen Daten nicht speichern! Überprüfe deine Internetverbindung, versuche es erneut und wende dich dann an den/die Administrator*in. Details siehe Konsole.");
                    error!(error.to_string());
                    self.change_selected_material = None;
                    true
                }
                UpdateMessage::Cancel => {
                    self.change_selected_material = None;
                    true
                }
            },
        }
    }
}

fn sort_material(material: &mut [MaterialTo]) {
    material.sort_by(|s, other| s.title.cmp(&other.title));
}

#[derive(Clone, PartialEq, Properties)]
pub struct MaterialUpdateButtonProperties {
    pub owner_id: i64,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(MaterialUpdateButton)]
pub fn update_button(props: &MaterialUpdateButtonProperties) -> Html {
    let props = props.clone();

    html! {
    <AdminOrOwner owner_id={props.owner_id}>
        <button type="button" class="btn btn-outline-danger btn-sm" data-bs-toggle="modal" data-bs-target={format!("#{MODAL_MATERIAL_UPDATE}")} onclick={props.onclick}> { "Ändern" } </button>
    </AdminOrOwner>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MaterialDeleteButtonProperties {
    pub owner_id: i64,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(MaterialDeleteButton)]
pub fn delete_button(props: &MaterialDeleteButtonProperties) -> Html {
    let props = props.clone();

    html! {
    <AdminOrOwner owner_id={props.owner_id}>
        <button type="button" class="btn btn-danger btn-sm" data-bs-toggle="modal" data-bs-target={format!("#{MODAL_MATERIAL_DELETE}")} onclick={props.onclick}> { "Löschen" } </button>
    </AdminOrOwner>
    }
}
