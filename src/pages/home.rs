use crate::{
    components::{modal::Modal, project_list::*},
    service::project::{create_project, get_pending_projects},
    utilities::requests::fetch::FetchError,
};
use chrono::{NaiveDateTime, Utc};
use gloo_console::{error, warn};
use gloo_dialogs::alert;
use gloo_utils::document;
use serde::Serialize;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Home {
    projects: Option<Vec<Project>>,
    project_due_date: NaiveDateTime,
    project_title: String,
    project_description: String,
}

pub enum Msg {
    ProjectsLoaded(Vec<Project>),
    ProjectsLoadError(FetchError),
    CreateProject(MouseEvent),
    AbortCreateProject(MouseEvent),
    NameInput(InputEvent),
    DescriptionInput(InputEvent),
    DateInput(Event),
    CreateProjectSuccess(Project),
    CreateProjectFail(FetchError),
    ProjectDeleted(i32),
}

#[derive(Clone, Serialize)]
pub struct CreateProjectBody {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            projects: None,
            project_due_date: Utc::now().naive_local(),
            project_title: "".to_string(),
            project_description: "".to_string(),
        }
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
                                <button class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#modalProjectCreate">{ "Neues Projekt" }</button>
                            </div>
                        </div>
                        if let Some(contributions) = self.projects.clone() {
                            <ProjectList projects={contributions} project_delete={ ctx.link().callback(Msg::ProjectDeleted) }/>
                        } else {
                            { "Abgaben werden geladen..." }
                        }
                    </div>
                </div>
                <Modal
                    title={"Projekt erstellen".to_string() }
                    id={ "modalProjectCreate".to_string() }
                    actions = { vec![
                        ("Abbrechen".to_string(), "btn btn-secondary".to_string(),  ctx.link().callback(Msg::AbortCreateProject)),
                        ("Erstellen".to_string(), "btn btn-danger".to_string(),  ctx.link().callback(Msg::CreateProject))
                        ]
                    }
                >
                <>
                    <script>
                    {"tinymce.init({
                        selector: '#textareaDescription',
                        setup: (editor) => {
                            editor.on('change', (e) => {
                                textareaDescription.textContent = e.target.contentDocument.body.innerHTML;
                            });
                        }
                    });"}
                    </script>
                    <form id="createProjectForm" class="">
                        <div class="row">
                            <div class="col">
                                <label for="inputCreateProjectTitle">{ "Name des Projektes" }</label>
                                <input id="inputCreateProjectTitle" type="text" class="form-control" placeholder="Name des Projektes" oninput={ ctx.link().callback(Msg::NameInput) }/>
                            </div>
                            <div class="col">
                                <label for="inputCreateProjectDueDate">{ "Abgabedatum" }</label>
                                <input id="inputCreateProjectDueDate" type="datetime-local" class="form-control" value={ self.project_due_date.to_string() } onchange={ ctx.link().callback(Msg::DateInput) }/>
                            </div>
                        </div>
                        <div class="row mt-2">
                            <div class="col">
                                <label for="textareaDescription">{ "Beschreibung" }</label>
                                <textarea id="textareaDescription" oninput={ ctx.link().callback(Msg::DescriptionInput) }></textarea>
                            </div>
                        </div>
                    </form>
                </>
                </Modal>
            </div>

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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProjectsLoaded(projects) => {
                self.projects = Some(projects);
                true
            }
            Msg::ProjectsLoadError(error) => {
                log_fetch_error(error);
                alert("Could not download list!");
                true
            }

            Msg::CreateProject(_) => {
                if self.project_title.is_empty() {
                    alert("Titel fehlt!");
                    return false;
                }

                let title = self.project_title.clone();
                let description = get_element_string_value("textareaDescription");
                let due_date = self.project_due_date;

                ctx.link().send_future(async move {
                    match create_project(title, description, due_date).await {
                        Ok(result) => Msg::CreateProjectSuccess(result),
                        Err(error) => Msg::CreateProjectFail(error),
                    }
                });

                false
            }
            Msg::NameInput(event) => {
                self.project_title = get_value_from_input_event(event);
                false
            }
            Msg::DescriptionInput(event) => {
                self.project_description = get_value_from_input_event(event);
                false
            }
            Msg::DateInput(event) => {
                let date_string = get_value_from_event(event);
                self.project_due_date =
                    match NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%S%.3f") {
                        Ok(date_time) => date_time,
                        Err(_e) => NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%S")
                            .unwrap_or_else(|_| {
                                NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M")
                                    .expect("problem")
                            }),
                    };
                false
            }
            Msg::CreateProjectSuccess(new_item) => {
                match &mut self.projects {
                    Some(projects) => projects.push(new_item),
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
            Msg::AbortCreateProject(_) => false,
        }
    }
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

fn get_value_from_input_event(e: InputEvent) -> String {
    // Shamelessly stolen yew example code. Conversions wont fail unless the element vanished into thin air or similar.
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

fn get_value_from_event(e: Event) -> String {
    // Shamelessly stolen yew example code. Conversions wont fail unless the element vanished into thin air or similar.
    let event_target = e.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

fn get_element_string_value(name: &str) -> String {
    let document = document();
    let element = document.get_element_by_id(name).unwrap_throw();
    element.text_content().unwrap_throw()
}
