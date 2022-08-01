use crate::{
    components::project::{
        project_list::ProjectList,
        project_modals::{ProjectCreateModal, MODAL_NEW_PROJECT},
    },
    service::project::{get_pending_projects, ProjectTo},
    utilities::requests::fetch::FetchError,
};

use gloo_console::{error, warn};
use gloo_dialogs::alert;
use gloo_utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

pub struct Home {
    projects: Option<Vec<ProjectTo>>,
}

pub enum Msg {
    ProjectsLoaded(Vec<ProjectTo>),
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
        Self { projects: None }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="container">
                <div class="row mt-2">
                    <div class="col">
                        <div class="row">
                            <div class="col">
                                <h2> { "Alle Abgaben" } </h2>
                            </div>
                            <div class="col text-end">
                                <button class="btn btn-danger" data-bs-toggle="modal" data-bs-target={format!("#{MODAL_NEW_PROJECT}")}>{ "Neues Projekt" }</button>
                            </div>
                        </div>
                        if let Some(contributions) = self.projects.clone() {
                            <ProjectList projects={contributions} project_delete={ ctx.link().callback(Msg::ProjectDeleted) } project_change={ ctx.link().callback(Msg::ProjectChanged)}/>
                        } else {
                            { "Abgaben werden geladen..." }
                        }
                    </div>
                </div>
            </div>
            <ProjectCreateModal on_success={ctx.link().callback(Msg::CreateProjectSuccess)} on_error={ctx.link().callback(Msg::CreateProjectFail)}/>

            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                match get_pending_projects().await {
                    Ok(contributions) => Msg::ProjectsLoaded(contributions),
                    Err(error) => Msg::ProjectsLoadError(error),
                }
            })
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProjectsLoaded(mut projects) => {
                sort_projects(&mut projects);
                self.projects = Some(projects);
                true
            }
            Msg::ProjectsLoadError(error) => {
                log_fetch_error(error);
                alert("Could not download list!");
                true
            }
            Msg::CreateProjectSuccess(new_item) => {
                match &mut self.projects {
                    Some(projects) => {
                        projects.push(new_item);
                        sort_projects(projects);
                    }
                    None => self.projects = Some(vec![new_item]),
                }
                true
            }
            Msg::CreateProjectFail(error) => {
                log_fetch_error(error);
                alert("Konnte Projekt nicht erstellen! Details ggfs. siehe Konsole.");
                false
            }
            Msg::ProjectDeleted(project_id) => match &mut self.projects {
                Some(projects) => {
                    projects.retain(|x| x.id != project_id);
                    true
                }
                None => {
                    warn!("Got delete project success event but project list was empty!");
                    false
                }
            },
            Msg::ProjectChanged(project) => {
                match &mut self.projects {
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

fn sort_projects(projects: &mut [ProjectTo]) {
    projects.sort_by(|p1, p2| p1.due.cmp(&p2.due));
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
    // Shamelessly stolen yew example code. Conversions wont fail unless the element vanished into thin air or similar.
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

pub fn get_value_from_event(e: Event) -> String {
    // Shamelessly stolen yew example code. Conversions wont fail unless the element vanished into thin air or similar.
    let event_target = e.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

pub fn get_input_text_content(name: &str) -> String {
    let document = document();
    let element: HtmlInputElement = document
        .get_element_by_id(name)
        .unwrap_throw()
        .dyn_into()
        .unwrap_throw();
    element.value()
}

pub fn get_selected_value(id_of_select: &str) -> String {
    let document = document();
    let element: HtmlSelectElement = document
        .get_element_by_id(id_of_select)
        .unwrap_throw()
        .dyn_into()
        .unwrap_throw();
    element.value()
}
