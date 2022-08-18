use gloo_console::{error, warn};
use gloo_dialogs::alert;

use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Properties};
use yew_router::prelude::Link;

use crate::{
    components::{
        delete_modal::DeleteModal,
        project::modals::{ProjectUpdateModal, MODAL_UPDATE_PROJECT},
    },
    service::project::{delete_project, ProjectTo},
    utilities::requests::fetch::FetchError,
    Route,
};

pub enum DeleteMessage {
    ListItemButtonClick(ProjectTo),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success(i32),
    Fail(FetchError),
}

pub enum UpdateMessage {
    Update(ProjectTo),
    Success(ProjectTo),
    Error(FetchError),
}

#[allow(clippy::enum_variant_names)] // Messages are similar, but that could change
pub enum Msg {
    Delete(DeleteMessage),
    Update(UpdateMessage),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectListProperties {
    pub(crate) projects: Vec<ProjectTo>,
    pub(crate) project_delete: Callback<i32>,
    pub(crate) project_change: Callback<ProjectTo>,
}

pub struct ProjectList {
    selected_delete: Option<ProjectTo>,
    selected_update: Option<ProjectTo>,
}

impl Component for ProjectList {
    type Message = Msg;

    type Properties = ProjectListProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            selected_delete: None,
            selected_update: None,
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
                            { "Bearbeiten" }
                        </th>
                    </tr>
                    </thead>
                        <tbody>
                        { for projects.iter().map(|project| {
                            let project_clone = project.clone();
                            let project_clone_2 = project.clone();
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
                                    <button class="btn btn-sm btn-outline-danger" onclick={ ctx.link().callback(move |_| Msg::Update(UpdateMessage::Update(project_clone_2.clone()))) }  data-bs-toggle="modal" data-bs-target={format!("#{MODAL_UPDATE_PROJECT}")}>{ "Bearbeiten" }</button>
                                </td>
                                <td>
                                    <button class="btn btn-sm btn-danger" onclick={ ctx.link().callback(move |_| Msg::Delete(DeleteMessage::ListItemButtonClick(project_clone.clone()))) }  data-bs-toggle="modal" data-bs-target="#modalProjectDelete">{ "Löschen" }</button>
                                </td>
                            </tr>
                            }})

                        }
                        </tbody>
                </table>
                <ProjectUpdateModal project={ self.selected_update.clone() } on_success={ ctx.link().callback(|project| Msg::Update(UpdateMessage::Success(project))) } on_error={ ctx.link().callback(|error| Msg::Update(UpdateMessage::Error(error))) }/>
                <DeleteModal
                    title={"Projekt löschen".to_string() }
                    id={ "modalProjectDelete".to_string() }
                    on_cancel={ ctx.link().callback(|x| Msg::Delete(DeleteMessage::AbortClick(x))) }
                    on_confirm={ ctx.link().callback(|x| Msg::Delete(DeleteMessage::AcceptClick(x))) }
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
            Msg::Delete(delete_message) => match delete_message {
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
                            Ok(()) => Msg::Delete(DeleteMessage::Success(project_id)),
                            Err(error) => Msg::Delete(DeleteMessage::Fail(error)),
                        }
                    });
                    false
                }
                DeleteMessage::AbortClick(_) => {
                    self.selected_delete = None;
                    false
                }
            },
            Msg::Update(message) => match message {
                UpdateMessage::Update(project) => {
                    self.selected_update = Some(project);
                    true
                }
                UpdateMessage::Success(updated_project) => {
                    ctx.props().project_change.emit(updated_project);
                    false
                }
                UpdateMessage::Error(error) => {
                    error!(error.to_string());
                    alert("Das Projekt konnte nicht angepasst werden. Überprüfe Deine Internetverbindung, versuche es erneut und wende dich sonst an den/die Administrator*in. Details siehe Konsole.");
                    false
                }
            },
        }
    }
}
