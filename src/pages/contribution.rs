use crate::{
    components::upload::Upload,
    utilities::requests::fetch::{get_request_struct, FetchError},
};
use serde::Deserialize;
use yew::{html, Component, Properties};

use gloo_console::error;
use gloo_dialogs::alert;

pub enum Msg {
    MetadataLoaded(Metadata),
    MetadataLoadError(FetchError),
}

#[derive(Deserialize)]
pub struct Metadata {
    heading: String,
    description: String,
    playbacks_audio: Vec<String>,
    playbacks_video: Vec<String>,
    scores: Vec<String>,
}
pub struct Contribution {
    metadata: Option<Metadata>,
}

#[derive(PartialEq, Properties)]
pub struct ContributionProperties {
    pub id: usize,
}

impl Component for Contribution {
    type Message = Msg;

    type Properties = ContributionProperties;

    fn create(ctx: &yew::Context<Self>) -> Self {
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
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Playbacks" }</h2>
                    </div>
                </div>
                <div class="row">
                    { for metadata.playbacks_audio.iter().map(|audio| html!{
                        <div class="col">
                            <audio controls=true id={audio.clone()} src={audio.clone()}></audio>
                        </div>
                    })}
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Videos" }</h2>
                    </div>
                </div>
                { for metadata.playbacks_video.iter().map(|video| html!{
                    <div class="row">
                        <div class="col">
                            <div class="ratio ratio-16x9">
                            <video controls=true>
                                <video id={video.clone()}></video>
                                    <source src={video.clone()}/>
                                </video>
                            </div>
                        </div>
                    </div>
                })}
                <div class="row mt-2">
                    <div class="col">
                            <h2>{ "Noten" }</h2>
                    </div>
                </div>
                <div class="row">
                { for metadata.scores.iter().map(|score| html!{
                    <div class="mt-2 ratio ratio-16x9">
                        <embed src={ format!("{}#toolbar=0&navpanes=0&scrollbar=0&statusbar=0&messages=0", score) }/>
                    </div>
                }) }
               </div>
                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Neue Datei hochladen" }</h2>
                        <Upload form_id="inputFileUpload" field_name="filetoupload2" target_url={format!("http://localhost:8001/contributions/{}", ctx.props().id)}/>
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
            let id = ctx.props().id;
            ctx.link().send_future(async move {
                match get_request_struct::<Metadata>(format!(
                    "http://localhost:8001/contributions/{}",
                    id
                ))
                .await
                {
                    Ok(metadata) => Msg::MetadataLoaded(metadata),
                    Err(error) => Msg::MetadataLoadError(error),
                }
            })
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
        }
    }
}
