use crate::{
    components::{
        jwt_context::get_token_data,
        project::{
            list::ProjectLists,
            modals::{ProjectCreateModal, MODAL_NEW_PROJECT},
        },
    },
    service::project::{get_all_projects, get_my_projects, get_pending_projects, ProjectTo},
    utilities::requests::fetch::FetchError,
};

use chrono::Utc;
use gloo_console::error;
use gloo_dialogs::alert;
use gloo_utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

pub struct Home {
    pending_projects: Option<Vec<ProjectTo>>,
    my_projects: Option<Vec<ProjectTo>>,
    all_projects: Option<Vec<ProjectTo>>,
}

pub enum Msg {
    AllProjectsLoaded(Vec<ProjectTo>),
    MyProjectsLoaded(Vec<ProjectTo>),
    PendingProjectsLoaded(Vec<ProjectTo>),
    ProjectsLoadError(FetchError),
    CreateProjectSuccess(ProjectTo),
    CreateProjectFail(FetchError),
    ProjectDeleted(i32),
    ProjectChanged(ProjectTo),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            pending_projects: None,
            my_projects: None,
            all_projects: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = get_token_data().unwrap_throw();
        html! {
            <>
            <div class="container">
                <div class="row mt-2">
                    <div class="col">
                        { "Angemeldet als: " }
                        { "Name: " }
                        <i>{ &user.name }</i>
                        { "; Stimme: " }
                        <i>{ user.section }</i>
                        { "; Id: " }
                        <i>{ user.user_id }</i>
                    </div>
                </div>

                <div class="row mt-2">
                    <div class="col">
                        <h2> { "Projekte" } </h2>
                    </div>
                    <div class="col text-end">
                        <button class="btn btn-danger" data-bs-toggle="modal" data-bs-target={format!("#{MODAL_NEW_PROJECT}")}>{ "Neues Projekt" }</button>
                    </div>
                </div>
                <ProjectLists
                    projects={self.pending_projects.clone()}
                    all_projects={self.all_projects.clone()}
                    my_projects={self.my_projects.clone()}
                    project_delete={ ctx.link().callback(Msg::ProjectDeleted) }
                    project_change={ ctx.link().callback(Msg::ProjectChanged)}
                />
            </div>
            <ProjectCreateModal
                on_success={ctx.link().callback(Msg::CreateProjectSuccess)}
                on_error={ctx.link().callback(Msg::CreateProjectFail)}
            />

            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                match get_pending_projects().await {
                    Ok(contributions) => Msg::PendingProjectsLoaded(contributions),
                    Err(error) => Msg::ProjectsLoadError(error),
                }
            });

            ctx.link().send_future(async {
                match get_my_projects().await {
                    Ok(contributions) => Msg::MyProjectsLoaded(contributions),
                    Err(error) => Msg::ProjectsLoadError(error),
                }
            });

            let user = get_token_data().unwrap_throw();

            if user.is_admin {
                ctx.link().send_future(async {
                    match get_all_projects().await {
                        Ok(contributions) => Msg::AllProjectsLoaded(contributions),
                        Err(error) => Msg::ProjectsLoadError(error),
                    }
                });
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PendingProjectsLoaded(mut projects) => {
                sort_projects(&mut projects);
                self.pending_projects = Some(projects);
                true
            }
            Msg::AllProjectsLoaded(mut projects) => {
                sort_projects(&mut projects);
                self.all_projects = Some(projects);
                true
            }
            Msg::MyProjectsLoaded(mut projects) => {
                sort_projects(&mut projects);
                self.my_projects = Some(projects);
                true
            }
            Msg::ProjectsLoadError(error) => {
                log_fetch_error(error);
                alert("Could not download list!");
                true
            }
            Msg::CreateProjectSuccess(project) => {
                if project.due > Utc::now().naive_local() {
                    add_and_sort(&mut self.pending_projects, &project);
                }

                add_and_sort(&mut self.my_projects, &project);
                add_and_sort(&mut self.all_projects, &project);

                true
            }
            Msg::CreateProjectFail(error) => {
                log_fetch_error(error);
                alert("Konnte Projekt nicht erstellen! Details ggfs. siehe Konsole.");
                false
            }
            Msg::ProjectDeleted(project_id) => {
                delete_project(&mut self.pending_projects, project_id);
                delete_project(&mut self.my_projects, project_id);
                delete_project(&mut self.all_projects, project_id);

                true
            }
            Msg::ProjectChanged(project) => {
                match &mut self.pending_projects {
                    Some(projects) => {
                        projects.retain(|x| x.id != project.id);
                        projects.push(project);
                        sort_projects(projects);
                    }
                    None => (),
                }
                true
            }
        }
    }
}

fn delete_project(vec: &mut Option<Vec<ProjectTo>>, id: i32) {
    if let Some(vec) = vec {
        vec.retain(|x| x.id != id)
    }
}

fn add_and_sort(vec: &mut Option<Vec<ProjectTo>>, project: &ProjectTo) {
    if let Some(vec) = vec {
        vec.push(project.clone());
        sort_projects(vec);
    }
}

fn sort_projects(projects: &mut [ProjectTo]) {
    projects.sort_unstable_by(|p1, p2| p1.due.cmp(&p2.due).then(p1.title.cmp(&p2.title)));
}

fn log_fetch_error(error: FetchError) {
    match error {
        FetchError::JsError(js_error) => error!(js_error),
        FetchError::WrongContentType => {
            error!("Content type of fetched object was wrong!")
        }
        FetchError::SerdeError(serde_error) => {
            error!("type could not be parsed!");
            error!(serde_error.to_string());
        }
        FetchError::StatusCode(status) => error!("Got status code {} from request!", status),
    }
}

pub fn get_value_from_input_event(e: InputEvent) -> String {
    let target: HtmlInputElement = e.target_dyn_into().unwrap_throw();
    target.value()
}

pub fn get_value_from_event(e: Event) -> String {
    // Shamelessly stolen yew example code. Conversions wont fail unless the element vanished into thin air or similar.
    let event_target = e.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

pub fn get_input_text_content(name: &str) -> String {
    let element: HtmlInputElement = document()
        .get_element_by_id(name)
        .unwrap_throw()
        .dyn_into()
        .unwrap_throw();
    element.value()
}

pub fn get_selected_value(id_of_select: &str) -> String {
    let element: HtmlSelectElement = document()
        .get_element_by_id(id_of_select)
        .unwrap_throw()
        .dyn_into()
        .unwrap_throw();
    element.value()
}
