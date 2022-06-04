use gloo_console::warn;
use gloo_dialogs::alert;
use serde::Deserialize;
use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Properties};
use yew_router::prelude::Link;

use crate::{service::project::delete_project, utilities::requests::fetch::FetchError, Route};

use crate::components::delete_modal::DeleteModal;

#[derive(Default, Deserialize, PartialEq, Clone)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub due: String,
}

pub enum DeleteMessage {
    ListItemButtonClick(Project),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success(i32),
    Fail(FetchError),
}

#[allow(clippy::enum_variant_names)] // Messages are similar, but that could change
pub enum Msg {
    DeleteMessage(DeleteMessage),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectListProperties {
    pub(crate) projects: Vec<Project>,
    pub(crate) project_delete: Callback<i32>,
}

pub struct ProjectList {
    selected_delete: Option<Project>,
}

impl Component for ProjectList {
    type Message = Msg;

    type Properties = ProjectListProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            selected_delete: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let projects = &ctx.props().projects;
        html! {
            <>
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>
                            { "Stück" }
                        </th>
                        <th>
                            { "Abgabe bis" }
                        </th>
                        <th>
                            { "Löschen" }
                        </th>
                    </tr>
                    </thead>
                        <tbody>
                        { for projects.iter().map(|project| {
                            let project_clone = project.clone();
                            html!{
                            <tr>
                                <td>
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::Event{id: project.id}}>
                                        { &project.title }
                                    </Link<Route>>
                                </td>
                                <td>
                                    { &project.due }
                                </td>
                                <td>
                                    <button class="btn btn-sm btn-outline-danger" onclick={ ctx.link().callback(move |_| Msg::DeleteMessage(DeleteMessage::ListItemButtonClick(project_clone.clone()))) }  data-bs-toggle="modal" data-bs-target="#modalProjectDelete">{ "Löschen" }</button>
                              
                                    </td>
                            </tr>
                            }})

                        }
                        </tbody>
                </table>
                <DeleteModal 
                    title={"Projekt löschen".to_string() } 
                    id={ "modalProjectDelete".to_string() } 
                    on_cancel={ ctx.link().callback(|x| Msg::DeleteMessage(DeleteMessage::AbortClick(x))) } 
                    on_confirm={ ctx.link().callback(|x| Msg::DeleteMessage(DeleteMessage::AcceptClick(x))) }
                >
                <>
                    <h4>
                        { "Warnung!" }
                    </h4>
                    <p>
                        { "Kann " }
                        <b>
                            { "nicht rückgängig " }
                        </b>
                        { "gemacht werden!" }
                    </p>
                    <p>
                        <b>
                            { "Alle Abgaben " }
                        </b>
                        { "werden " }   
                        <b>
                            { "unwiederruflich " }
                        </b>
                        { " gelöscht!" }
                    </p>
                    <p>
                        { "Das Projekt " }  
                        <i>if let Some(project) = &self.selected_delete { 
                            { &project.title } 
                        } 
                        else { 
                            { "Fehler! kein Projekt ausgewählt" }
                        }
                        </i>
                        { " wirklich löschen?" }
                    </p>
                </>
                </DeleteModal>
                </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DeleteMessage(delete_message) => match delete_message {
                DeleteMessage::ListItemButtonClick(project) => {
                    self.selected_delete = Some(project);
                    true
                }
                DeleteMessage::Success(project_id) => {
                    ctx.props().project_delete.emit(project_id);
                    false
                }

                DeleteMessage::Fail(error) => {
                    match error {
                        FetchError::StatusCode(code) => {
                            if code == 404 {
                                alert("Project was already deleted!");
                                warn!("Could not delete project, got 404.");
                                return false;
                            }
                            warn!(format!(
                                "Could not delete project, got status code {}",
                                code
                            ))
                        }
                        _ => warn!("Could not delete project!"),
                    }
                    alert("Das Projekt konnte nicht gelöscht werden! Details siehe Konsole.");
                    warn!(format!("{:?}", error));
                    false
                }
                DeleteMessage::AcceptClick(_) => {
                    let project_id = match self.selected_delete.take() {
                        Some(project) => project.id,
                        None => {
                            alert(
                            "Fehler: kein ausgewähltes Projekt gefunden. Bitte erneut versuchen.",
                        );
                            return false;
                        }
                    };
                    ctx.link().send_future(async move {
                        let result = delete_project(project_id).await;
                        match result {
                            Ok(()) => Msg::DeleteMessage(DeleteMessage::Success(project_id)),
                            Err(error) => Msg::DeleteMessage(DeleteMessage::Fail(error)),
                        }
                    });
                    false
                }
                DeleteMessage::AbortClick(_) => {
                    self.selected_delete = None;
                    false
                }
            }
        }
    }
}
