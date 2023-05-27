use gloo_console::{error, warn};
use gloo_dialogs::alert;

use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Component, Html, Properties};
use yew_router::prelude::Link;

use crate::{
    components::{
        admin_only::AdminOrOwner,
        delete_modal::DeleteModal,
        project::modals::{ProjectUpdateModal, MODAL_UPDATE_PROJECT},
    },
    service::project::{delete_project, ProjectTo},
    utilities::{date::format_datetime_human_readable, requests::fetch::FetchError},
    Route,
};

pub enum DeleteMessage {
    ListItemButtonClick(ProjectTo),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success(i64),
    Fail(FetchError),
}

pub enum UpdateMessage {
    Update(ProjectTo),
    Success(ProjectTo),
    Error(FetchError),
}

pub enum Msg {
    Delete(DeleteMessage),
    Update(UpdateMessage),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectListsProperties {
    pub(crate) projects: Option<Vec<ProjectTo>>,
    pub(crate) all_projects: Option<Vec<ProjectTo>>,
    pub(crate) my_projects: Option<Vec<ProjectTo>>,
    pub(crate) project_delete: Callback<i64>,
    pub(crate) project_change: Callback<ProjectTo>,
}

pub struct ProjectLists {
    selected_delete: Option<ProjectTo>,
    selected_update: Option<ProjectTo>,
}

impl Component for ProjectLists {
    type Message = Msg;

    type Properties = ProjectListsProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            selected_delete: None,
            selected_update: None,
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
                    alert("Das Projekt konnte nicht angepasst werden. \
                    Überprüfe Deine Internetverbindung, versuche es erneut und wende dich sonst an den/die Administrator*in. \
                    Details siehe Konsole.");
                    false
                }
            },
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
            if let Some(projects) = &ctx.props().projects {
            <div class="row mt-2">
                <div class="col">
                    <h3> { "Ausstehende Projekte" } </h3>
                </div>
            </div>
            <ProjectList
                projects={projects.clone()}
                on_change={ ctx.link().callback(move |project| Msg::Update(UpdateMessage::Update(project)))}
                on_delete={ ctx.link().callback(move |project| Msg::Delete(DeleteMessage::ListItemButtonClick(project)))}
            />
            }

            if let Some(projects) = &ctx.props().all_projects {
            <div class="row mt-2">
                <div class="col">
                    <h3> { "Alle Projekte" } </h3>
                </div>
            </div>
            <ProjectList
                projects={projects.clone()}
                on_change={ ctx.link().callback(move |project| Msg::Update(UpdateMessage::Update(project)))}
                on_delete={ ctx.link().callback(move |project| Msg::Delete(DeleteMessage::ListItemButtonClick(project)))}
            />
            }

            if let Some(projects) =  &ctx.props().my_projects {
            <div class="row mt-2">
                <div class="col">
                    <h3> { "Meine Projekte" } </h3>
                </div>
            </div>
            <ProjectList
                projects={projects.clone()}
                on_change={ ctx.link().callback(move |project| Msg::Update(UpdateMessage::Update(project)))}
                on_delete={ ctx.link().callback(move |project| Msg::Delete(DeleteMessage::ListItemButtonClick(project)))}
            />
            }

            <ProjectUpdateModal
                project={ self.selected_update.clone() }
                on_success={ ctx.link().callback(|project| Msg::Update(UpdateMessage::Success(project))) }
                on_error={ ctx.link().callback(|error| Msg::Update(UpdateMessage::Error(error))) }
            />

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
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct ProjectListProperties {
    pub(crate) projects: Vec<ProjectTo>,
    pub(crate) on_delete: Callback<ProjectTo>,
    pub(crate) on_change: Callback<ProjectTo>,
}

#[function_component(ProjectList)]
fn project_list(props: &ProjectListProperties) -> Html {
    let props = props.clone();
    let project_change = props.on_change.clone();
    html! {
        <div class="table-responsive">
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>
                            { "Stück" }
                        </th>
                        <th>
                            { "Abgabe bis" }
                        </th>
                        <th colspan="2">
                            { "Bearbeiten" }
                        </th>
                    </tr>
                    </thead>
                        <tbody>
                        if props.projects.is_empty() {
                            <td>{ "Es gibt keine Projekte in dieser Kategorie." }</td>
                        }
                        { for props.projects.iter().map(|project| {
                            let project_change = project_change.clone();
                            let project_delete = props.on_delete.clone();
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
                                    { format_datetime_human_readable(&project.due) }
                                </td>
                                <AdminOrOwner owner_id={ project.creator }>
                                <td>
                                    <button
                                        class="btn btn-sm btn-outline-danger"
                                        onclick={ move |_| project_change.emit(project_clone.clone()) }
                                        data-bs-toggle="modal"
                                        data-bs-target={format!("#{MODAL_UPDATE_PROJECT}")}
                                    >
                                        { "Bearbeiten" }
                                    </button>
                                </td>
                                <td>
                                    <button
                                        class="btn btn-sm btn-danger"
                                        onclick={ move |_| project_delete.emit(project_clone_2.clone()) }
                                        data-bs-toggle="modal" data-bs-target="#modalProjectDelete"
                                    >
                                        { "Löschen" }
                                    </button>
                                </td>
                                </AdminOrOwner>
                            </tr>
                            }})

                        }
                        </tbody>
                </table>
            </div>
    }
}
