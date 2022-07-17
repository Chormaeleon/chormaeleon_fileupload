use gloo_console::{error, warn};
use gloo_dialogs::alert;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Component, Properties};

use crate::{
    components::{admin_only::AdminOnly, delete_modal::DeleteModal, modal::Modal, upload::Upload},
    service::material::{
        delete_material, material_by_project, material_upload_url, material_url, MaterialCategory,
        MaterialTo,
    },
    utilities::requests::fetch::FetchError,
};

pub struct Material {
    pub material: Vec<MaterialTo>,
    delete_selected_material: Option<MaterialTo>,
}

pub enum Msg {
    MaterialUploadSuccess(String),
    MaterialUploadError(String),
    MaterialFetchSuccess(Vec<MaterialTo>),
    MaterialFetchError(FetchError),
    Delete(DeleteMessage),
}

pub enum DeleteMessage {
    DeleteButtonClick(MaterialTo),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success,
    Fail(FetchError),
}

#[derive(Clone, PartialEq, Properties)]
pub struct MaterialProperties {
    pub id: i32,
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
            delete_selected_material: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let material = &self.material;
        html! {
            <>
            <div class="row mt-2">
                <div class="col text-end">
                    <button type="button" class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#uploadMaterialModal">
                        { "Übungsmaterial hinzufügen" }
                    </button>
                </div>
            </div>
            <div class="row mt-2">
                <div class="col">
                    <h2>{ "Playbacks" }</h2>
                </div>
            </div>
            <div class="row">
            {
                for material.iter().filter(|x| x.category == MaterialCategory::Audio).map(|audio|  {
                    let clone = audio.clone();
                    html!{
                    <div class="col">
                        <h5> { &audio.title } </h5>
                        <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(ctx.props().id, &audio.file_technical_name) }></audio>
                        <h6> <i> { &audio.file_name } </i> </h6>
                        <MaterialDeleteButton
                            onclick={
                                ctx.link().callback(move |_|
                                    Msg::Delete(DeleteMessage::DeleteButtonClick(clone.clone()))
                                )
                            }
                        />
                    </div>
                    }
                })
            }
            </div>
            <div class="row mt-2">
                <div class="col">
                    <h2>{ "Videos" }</h2>

                    { for material.iter().filter(|x| x.category == MaterialCategory::Video).map(|video| {
                        let video = video.clone();
                        html!{
                            <>
                            <div class="row">
                                <div class="col">
                                <h5> { &video.title } </h5>
                                <div class="ratio ratio-16x9">
                                        <video id={video.title.clone()} controls=true>
                                            <source src={ material_url(ctx.props().id, &video.file_technical_name) }/>
                                        </video>
                                    </div>
                                    <h6> <i> { &video.file_name } </i> </h6>
                                </div>
                            </div>
                            <MaterialDeleteButton
                                onclick={
                                    ctx.link().callback(move |_|
                                        Msg::Delete(DeleteMessage::DeleteButtonClick(video.clone()))
                                    )
                                }
                            />
                            </>
                        }
                        })
                    }

                </div>
            </div>
            <div class="row mt-2">
                <div class="col">
                    <h2>{ "Noten" }</h2>
                    { for material.iter().filter(|x| x.category == MaterialCategory::SheetMusic).map(|score| {
                        let score = score.clone();
                        html!{
                            <>
                            <div class="row">
                                <div class="col">
                                    <h5> { &score.title } </h5>
                                    <div class="mt-2 ratio ratio-16x9">
                                        <embed src={ format!("{}#toolbar=0&navpanes=0&scrollbar=0&statusbar=0&messages=0", material_url(ctx.props().id, &score.file_technical_name)) }/>
                                    </div>
                                    <h6> <i> { &score.file_name } </i> </h6>
                                </div>
                            </div>
                            <MaterialDeleteButton
                                onclick={
                                    ctx.link().callback(move |_|
                                        Msg::Delete(DeleteMessage::DeleteButtonClick(score.clone()))
                                    )
                                }
                            />
                            </>
                        }
                        })
                    }
                </div>
            </div>

            <div class="row mt-2">
                <div class="col">
                    <h2>{ "Sonstige Dateien + Downloads" }</h2>

                    <table class="table table-striped">
                        <thead>
                            <tr>
                                <th>
                                    { "Dateibeschreibung" }
                                </th>
                                <th>
                                    { "Link" }
                                </th>
                                <AdminOnly>
                                    <th>
                                        { "Löschen" }
                                    </th>
                                </AdminOnly>
                            </tr>
                        </thead>
                        <tbody>
                        { for material.iter().filter(|x| x.category == MaterialCategory::Other).map(|other| {
                            let other = other.clone();
                            html!{
                            <tr>
                                <td>
                                    { &other.title }
                                </td>
                                <td>
                                    <a href={ material_url(ctx.props().id, &other.file_technical_name) } download={ other.file_name.clone().to_string() }> { &other.file_name } </a>
                                </td>
                                <AdminOnly>
                                    <td>
                                        <MaterialDeleteButton
                                            onclick={
                                                ctx.link().callback(move |_|
                                                    Msg::Delete(DeleteMessage::DeleteButtonClick(other.clone()))
                                                )
                                            }
                                        />
                                    </td>
                                </AdminOnly>
                            </tr>
                        } }) }
                        </tbody>
                    </table>
                </div>
            </div>

            <Modal
                title={"Übungsmaterial hochladen".to_string() }
                id={ "uploadMaterialModal".to_string() }
                actions = { vec![] }
            >
            <form id="inputMaterialUpload" class="" name="formUpload" enctype="multipart/form-data">
            <div class="row">
                <div class="col">
                    <label for="inputMaterialTitle">{ "Name für die Datei" }</label>
                    <input id="inputMaterialTitle" type="text" class="form-control" name="title" placeholder="Titel der Datei"/>
                </div>
                    <div class="col">
                        <label for="selectMaterialCategory">{ "Art der Datei (Playback, ...)" }</label>
                        <select id="selectMaterialCategory" name="material_category" class="form-control" form="inputMaterialUpload">
                            <option value="Audio">{ "Audio" }</option>
                            <option value="Video">{ "Video" }</option>
                            <option value="Sheet">{ "Noten" }</option>
                            <option value="Other">{ "Sonstiges" }</option>
                        </select>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <Upload form_id="inputMaterialUpload" field_name="file" target_url={ material_upload_url(ctx.props().id) } multiple=false success_callback={ ctx.link().callback(Msg::MaterialUploadSuccess) }  failure_callback={ ctx.link().callback(Msg::MaterialUploadError) } />
                    </div>
                </div>
            </form>
            </Modal>

            <DeleteModal id="modalMaterialDelete" title="Material wirklich löschen?" on_cancel={ ctx.link().callback(|e| Msg::Delete(DeleteMessage::AbortClick(e))) } on_confirm={ ctx.link().callback(|e| Msg::Delete(DeleteMessage::AcceptClick(e)))  }>
                if let Some(mat) = &self.delete_selected_material {
                    <p> { "Beschreibung: " } { &mat.title } </p>
                    <p> { "Dateiname: " } <i> { &mat.file_name } </i> </p>
                } else {
                    { "Kein zu löschendes Element ausgewählt!" }
                }
            </DeleteModal>
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
                true
            }
            Msg::MaterialFetchError(error) => {
                error!(format!("Could not fetch material! Error: {}", error));
                alert("Fehler: Konnte das Material nicht laden. Überprüfe Deine Internetverbindung, lade die Seite neu und wende Dich ansonsten an den/die Administrator*in.");
                false
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MaterialDeleteButtonProperties {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(MaterialDeleteButton)]
pub fn button(props: &MaterialDeleteButtonProperties) -> Html {
    let props = props.clone();
    html! {
    <AdminOnly>
        <button type="button" class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#modalMaterialDelete" onclick={props.onclick}> { "Löschen" } </button>
    </AdminOnly>
    }
}
