use crate::{
    components::upload::Upload,
    utilities::requests::fetch::{get_request_struct, FetchError},
};
use serde::Deserialize;
use yew::{html, Component, Properties};

use chrono::NaiveDateTime;

use gloo_console::error;
use gloo_dialogs::alert;

pub enum Msg {
    MetadataLoaded(Project),
    MetadataLoadError(FetchError),
    MetadataUpload(String),
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct Project {
    pub heading: String,
    pub description: String,
    pub materials_audio: Vec<MetadataEntry>,
    pub materials_video: Vec<MetadataEntry>,
    pub materials_sheet: Vec<MetadataEntry>,
    pub materials_other: Vec<MetadataEntry>,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct MetadataEntry {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub file_name: String,
    pub file_technical_name: String,
    pub creator: i32,
    pub upload_at: NaiveDateTime,
}

pub struct Contribution {
    metadata: Option<Project>,
}

#[derive(PartialEq, Properties)]
pub struct ContributionProperties {
    pub id: usize,
}

impl Component for Contribution {
    type Message = Msg;

    type Properties = ContributionProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { metadata: None }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.metadata {
            Some(metadata) => html! {
                <>
               
                <div class="row mt-2">
                    <div class="col">
                        <h1>{ &metadata.heading }</h1>
                        <iframe srcdoc={metadata.description.clone()}></iframe>
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <a href="http://localhost:8080"> <button type="button" class="btn btn-outline-danger"> { "zurück" } </button></a>
                    </div>
                    <div class="col">
                        <button type="button" class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#uploadMaterialModal">
                            { "Übungsmaterial hinzufügen" }
                        </button>
                    </div>
                    <div class="col">
                       <a href={format!("http://localhost:8001/projects/{}/allSubmissions", ctx.props().id)} download="true">  <button class="btn btn-danger">{ "Abgaben downloaden" } </button></a>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Playbacks" }</h2>
                    </div>
                </div>
                <div class="row">
                    { for metadata.materials_audio.iter().map(|audio| html!{
                        <div class="col">
                            <h5> { &audio.title } </h5>
                            <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(ctx.props().id, &audio.file_technical_name) }></audio>
                            <h6> <i> { &audio.file_name } </i> </h6>
                        </div>
                    })}
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Videos" }</h2>
                  
                        { for metadata.materials_video.iter().map(|video| html!{
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
                        })}

                        </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Noten" }</h2>
                        { for metadata.materials_sheet.iter().map(|score| html!{
                            <div class="row">
                                <div class="col">
                                    <h5> { &score.title } </h5>
                                    <div class="mt-2 ratio ratio-16x9">
                                        <embed src={ format!("{}#toolbar=0&navpanes=0&scrollbar=0&statusbar=0&messages=0", material_url(ctx.props().id, &score.file_technical_name)) }/>
                                    </div>
                                    <h6> <i> { &score.file_name } </i> </h6>
                                </div>
                            </div>
                        }) }
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
                                        </tr>
                                    </thead>
                                    <tbody>
                                    { for metadata.materials_other.iter().map(|other| html!{
                                        <tr>
                                            <td>
                                                { &other.title }
                                            </td>
                                            <td>
                                                <a href={ material_url(ctx.props().id, &other.file_technical_name) } download={ other.file_name.clone()[..other.file_name.len() - 5].to_string() }> { &other.file_name } </a>
                                            </td>
                                        </tr>
                                    }) }
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
                                    <Upload form_id="inputContentUpload" field_name="file" target_url={format!("http://localhost:8001/projects/{}", ctx.props().id) } multiple=true success_callback={ ctx.link().callback(Msg::MetadataUpload) }/>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
                <div class="modal fade" id="uploadMaterialModal" tabindex="-1" aria-labelledby="uploadMaterialModalLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="uploadMaterialModalLabel">{ "Material hochladen" }</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
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
                                                <Upload form_id="inputMaterialUpload" field_name="file" target_url={format!("http://localhost:8001/projects/{}/material", ctx.props().id) } multiple=false success_callback={ ctx.link().callback(Msg::MetadataUpload) }/>
                                            </div>
                                        </div>
                                    </form>
                            </div>
                            ////<div class="modal-footer">
                            //    <button type="button" class="btn btn-primary" data-bs-dismiss="modal">{ "Close" }</button>
                            //</div>
                        </div>
                    </div>
                </div>
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
                }
                true
            }
            Msg::MetadataUpload(_text) => {
                load_data(ctx);
                true
            }
        }
    }
}

fn load_data(ctx: &yew::Context<Contribution>) {
    let id = ctx.props().id;
    ctx.link().send_future(async move {
        match get_request_struct::<Project>(format!(
            "http://localhost:8001/projects/{}",
            id
        ))
        .await
        {
            Ok(metadata) => Msg::MetadataLoaded(metadata),
            Err(error) => Msg::MetadataLoadError(error),
        }
    })
}

fn material_url(id: usize, file_technical_name: &str) -> String {
    format!(
        "http://localhost:8001/materials/{}/{}",
        id, file_technical_name
    )
}
