use std::error;

use crate::{
    components::{
        admin_only::AdminOnly, delete_modal::DeleteModal, iframe::IFrame, modal::Modal,
        submission_list::SubmissionList, upload::Upload,
    },
    service::{
        material::{
            delete_material, material_upload_url, material_url, MaterialKind, MetadataEntry,
        },
        project::{
            all_submissions_download_key, all_submissions_link, project_data,
            submission_upload_url, ProjectTo,
        },
        submission::{submissions_by_project, submissions_by_project_and_user, Submission},
    },
    utilities::{download_from_link, requests::fetch::FetchError},
};

use wasm_bindgen::UnwrapThrowExt;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Component, Properties};

use gloo_console::{error, warn};
use gloo_dialogs::alert;

pub enum Msg {
    MetadataLoaded(ProjectTo),
    MetadataLoadError(FetchError),
    MetadataUpload(String),
    AllSubmissionsLoaded(Vec<Submission>),
    MySubmissionsLoaded(Vec<Submission>),
    SubmissionsLoadError(FetchError),
    SubmissionsAdminLoadError(FetchError),
    SubmissionDeleted(i32),
    SubmissionUploaded(String),
    ProjectDownloadClick,
    ProjectDownloadKeyLoaded(String),
    Delete(DeleteMessage),
}

pub struct ProjectComponent {
    metadata: Option<ProjectTo>,
    all_submissions: Option<Vec<Submission>>,
    my_submissions: Vec<Submission>,
    delete_selected_material: Option<(MaterialKind, MetadataEntry)>,
}

pub enum DeleteMessage {
    DeleteButtonClick(MaterialKind, MetadataEntry),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success,
    Fail(FetchError),
}

#[derive(PartialEq, Properties)]
pub struct ProjectProperties {
    pub id: i32,
}

impl Component for ProjectComponent {
    type Message = Msg;

    type Properties = ProjectProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            metadata: None,
            all_submissions: None,
            my_submissions: Vec::new(),
            delete_selected_material: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.metadata {
            Some(metadata) => html! {
                <>
                <div class="row mt-2">
                    <div class="col">
                        <h1>{ &metadata.heading }</h1>
                        <IFrame content={metadata.description.clone()}/>
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <a href="/"> <button type="button" class="btn btn-outline-danger"> { "zurück" } </button></a>
                    </div>
                    <div class="col">
                        <button type="button" class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#uploadMaterialModal">
                            { "Übungsmaterial hinzufügen" }
                        </button>
                    </div>
                    <div class="col">
                       <button class="btn btn-danger" onclick={ ctx.link().callback(|_| Msg::ProjectDownloadClick) } >{ "Abgaben downloaden" } </button>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Playbacks" }</h2>
                    </div>
                </div>
                <div class="row">
                    {
                        for metadata.materials_audio.iter().map(|audio|  {
                            let clone = audio.clone();
                            html!{
                            <div class="col">
                                <h5> { &audio.title } </h5>
                                <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(ctx.props().id, &audio.file_technical_name) }></audio>
                                <h6> <i> { &audio.file_name } </i> </h6>
                                <MaterialDeleteButton
                                    onclick={
                                        ctx.link().callback(move |_|
                                            Msg::Delete(DeleteMessage::DeleteButtonClick(MaterialKind::Audio, clone.clone()))
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

                        { for metadata.materials_video.iter().map(|video| {
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
                                            Msg::Delete(DeleteMessage::DeleteButtonClick(MaterialKind::Video, video.clone()))
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
                        { for metadata.materials_sheet.iter().map(|score| {
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
                                            Msg::Delete(DeleteMessage::DeleteButtonClick(MaterialKind::SheetMusic, score.clone()))
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
                            { for metadata.materials_other.iter().map(|other| {
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
                                                        Msg::Delete(DeleteMessage::DeleteButtonClick(MaterialKind::Other, other.clone()))
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
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Neue Datei hochladen" }</h2>
                        <form id="inputContentUpload" class="" name="formMaterial" enctype="multipart/form-data">
                            <div class="row">
                                <div class="col">
                                    <label for="inputContentTitle"> { "Anmerkungen" } </label>
                                    <input id="inputContentTitle" type="text" class="form-control" name="note" maxlength="100" placeholder="z.B. Takt 15 bitte rausschneiden..."/>
                                </div>
                            </div>
                            <div class="row mt-2">
                                <div class="col">
                                    <Upload form_id="inputContentUpload" field_name="file" target_url={ submission_upload_url(ctx.props().id) } multiple=true success_callback={ ctx.link().callback(Msg::SubmissionUploaded) }/>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h4>{ "Meine Abgaben" }</h4>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <SubmissionList id="list1" submissions={ self.my_submissions.clone() } submission_delete={ ctx.link().callback(Msg::SubmissionDeleted) }/>
                    </div>
                </div>
                if let Some(all_submissions) = &self.all_submissions {
                    <div class="row mt-2">
                        <div class="col">
                            <h4>{ "Alle Abgaben" }</h4>
                        </div>
                    </div>
                    <div class="row mt-2">
                        <div class="col">
                            <SubmissionList id="list2" submissions={ all_submissions.clone() } submission_delete={ ctx.link().callback(Msg::SubmissionDeleted) }/>
                        </div>
                    </div>
                }

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
                            <label for="selectMaterialKind">{ "Art der Datei (Playback, ...)" }</label>
                            <select id="selectMaterialKind" name="material_kind" class="form-control" form="inputMaterialUpload">
                                <option value="Audio">{ "Audio" }</option>
                                <option value="Video">{ "Video" }</option>
                                <option value="Sheet">{ "Noten" }</option>
                                <option value="Other">{ "Sonstiges" }</option>
                            </select>
                        </div>
                    </div>
                    <div class="row mt-2">
                        <div class="col">
                            <Upload form_id="inputMaterialUpload" field_name="file" target_url={ material_upload_url(ctx.props().id) } multiple=false success_callback={ ctx.link().callback(Msg::MetadataUpload) }/>
                        </div>
                    </div>
                </form>
                </Modal>


                <DeleteModal id="modalMaterialDelete" title="Material wirklich löschen?" on_cancel={ ctx.link().callback(|e| Msg::Delete(DeleteMessage::AbortClick(e))) } on_confirm={ ctx.link().callback(|e| Msg::Delete(DeleteMessage::AcceptClick(e)))  }>
                    if let Some(mat) = &self.delete_selected_material {
                        <p> { "Beschreibung: " } { &mat.1.title } </p>
                        <p> { "Dateiname: " } <i> { &mat.1.file_name } </i> </p>
                    } else {
                        { "Kein zu löschendes Element ausgewählt!" }
                    }
                </DeleteModal>

                </>
            },
            None => html! {
                <h2>{ "Daten werden geladen" }</h2>
            },
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            load_data(ctx);

            let project_id = ctx.props().id;
            ctx.link().send_future(async move {
                let submissions = submissions_by_project(project_id).await;

                match submissions {
                    Ok(contributions) => Msg::AllSubmissionsLoaded(contributions),
                    Err(error) => Msg::SubmissionsAdminLoadError(error),
                }
            });

            ctx.link().send_future(async move {
                let submissions = submissions_by_project_and_user(project_id, 1).await;

                match submissions {
                    Ok(contributions) => Msg::MySubmissionsLoaded(contributions),
                    Err(error) => Msg::SubmissionsLoadError(error),
                }
            });
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MetadataLoaded(metadata) => {
                self.metadata = Some(metadata);
                true
            }
            Msg::MetadataLoadError(error) => {
                alert("Could not download data!");
                match error {
                    FetchError::JsError(js_error) => error!(js_error),
                    FetchError::WrongContentType => {
                        error!("Content type of fetched object was wrong!")
                    }
                    FetchError::SerdeError(serde_error) => {
                        error!("type could not be parsed!");
                        error!(serde_error.to_string());
                    }
                    FetchError::StatusCode(status) => {
                        error!("Got status {} while downloading metadata", status);
                    }
                }
                true
            }
            Msg::MetadataUpload(_text) => {
                load_data(ctx);
                true
            }
            Msg::MySubmissionsLoaded(submissions) => {
                self.my_submissions = submissions;
                true
            }
            Msg::AllSubmissionsLoaded(submissions) => {
                self.all_submissions = Some(submissions);
                true
            }
            Msg::SubmissionsLoadError(error) => {
                if let FetchError::StatusCode(status) = error {
                    if status == 401 {
                        return false;
                    }
                }
                gloo_console::error!(format!("{:?}", error));
                alert("Could not get submissions! For more info see the console log.");

                false
            }
            Msg::SubmissionsAdminLoadError(error) => {
                gloo_console::debug!(format!("could not get admin submissions {:?}", error));
                false
            }
            Msg::SubmissionDeleted(id) => {
                self.my_submissions.retain(|submission| submission.id != id);
                if let Some(all_submissions) = &mut self.all_submissions {
                    all_submissions.retain(|submission| submission.id != id);
                }
                true
            }
            Msg::SubmissionUploaded(text) => {
                let submission: Submission = serde_json::from_str(&text).unwrap_throw();
                if let Some(submissions) = &mut self.all_submissions {
                    submissions.push(submission.clone());
                }
                self.my_submissions.push(submission);
                true
            }
            Msg::ProjectDownloadClick => {
                let project_id = ctx.props().id;
                ctx.link().send_future(async move {
                    match all_submissions_download_key(project_id).await {
                        Ok(key) => Msg::ProjectDownloadKeyLoaded(key),
                        Err(error) => Msg::MetadataLoadError(error),
                    }
                });
                false
            }
            Msg::ProjectDownloadKeyLoaded(key) => {
                download_from_link(&all_submissions_link(ctx.props().id, key));
                false
            }
            Msg::Delete(message) => match message {
                DeleteMessage::DeleteButtonClick(category, item) => {
                    self.delete_selected_material = Some((category, item));
                    true
                }
                DeleteMessage::AcceptClick(_) => {
                    let material_id = match &self.delete_selected_material {
                        Some(m) => m.1.id,
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
                    let (category, item) = match self.delete_selected_material.take() {
                        Some(content) => content,
                        None => {
                            error!("Succesfully deleted material, but no item was found!");
                            return true;
                        }
                    };

                    let meta = match &mut self.metadata {
                        None => {
                            error!("No metadata found after deleting material!");
                            return false;
                        }
                        Some(m) => m,
                    };

                    let list = match category {
                        MaterialKind::Audio => &mut meta.materials_audio,
                        MaterialKind::Video => &mut meta.materials_video,
                        MaterialKind::SheetMusic => &mut meta.materials_sheet,
                        MaterialKind::Other => &mut meta.materials_other,
                    };

                    list.retain(|entry| entry.id != item.id);

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
        }
    }
}

fn load_data(ctx: &yew::Context<ProjectComponent>) {
    let project_id = ctx.props().id;
    ctx.link().send_future(async move {
        match project_data(project_id).await {
            Ok(metadata) => Msg::MetadataLoaded(metadata),
            Err(error) => Msg::MetadataLoadError(error),
        }
    })
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
