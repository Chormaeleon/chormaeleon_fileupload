use crate::{
    components::contribution_list::*,
    utilities::requests::fetch::{get_request_struct, post_request_struct, FetchError},
};
use chrono::{NaiveDateTime, Utc};
use gloo_console::error;
use gloo_dialogs::alert;
use serde::Serialize;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct Home {
    contributions: Option<Vec<ContributionListItem>>,
    project_due_date: NaiveDateTime,
    project_title: String,
    project_description: String,
}

pub enum Msg {
    ContributionsLoaded(Vec<ContributionListItem>),
    ContributionsLoadError(FetchError),
    CreateProject,
    NameInput(InputEvent),
    DescriptionInput(InputEvent),
    DateInput(Event),
    CreateProjectSuccess(ContributionListItem),
    CreateProjectFail(FetchError),
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
            contributions: None,
            project_due_date: Utc::now().naive_local(),
            project_title: "".to_string(),
            project_description: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <script>
            {"tinymce.init({
                selector: '#textareaDescription'
            });"}
            </script>
            <div class="container">
                <div class="row mt-2">
                    <div class="col">
                        <div class="row">
                            <div class="col">
                                <h2> { "Alle Abgaben" } </h2>
                            </div>
                            <div class="col text-end">
                                <button class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#createProjectModal">{ "Neues Projekt" }</button>
                            </div>
                        </div>
                        if let Some(contributions) = self.contributions.clone() {
                            <ContributionList contributions={contributions}/>
                        } else {
                            { "Abgaben werden geladen..." }
                        }
                    </div>
                </div>
                <div class="modal fade" id="createProjectModal" tabindex="-1" aria-labelledby="createProjectModalLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="createProjectModalLabel">{ "Projekt erstellen" }</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
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
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-outline btn-outline-danger" data-bs-dismiss="modal">{ "Abbrechen" }</button>
                                <button type="button" class="btn btn-danger" data-bs-dismiss="modal" onclick={ ctx.link().callback(|_| Msg::CreateProject) } >{ "Erstellen" }</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                match get_request_struct::<Vec<ContributionListItem>>(
                    "http://localhost:8001/pendingProjects".to_string(),
                )
                .await
                {
                    Ok(contributions) => Msg::ContributionsLoaded(contributions),
                    Err(error) => Msg::ContributionsLoadError(error),
                }
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContributionsLoaded(contribs) => {
                self.contributions = Some(contribs);
                true
            }
            Msg::ContributionsLoadError(error) => {
                log_fetch_error(error);
                alert("Could not download list!");
                true
            }

            Msg::CreateProject => {
                if self.project_title.is_empty() {
                    alert("Titel fehlt!");
                    return false;
                }

                let title = self.project_title.clone();
                let description = self.project_description.clone();
                let due_date = self.project_due_date;

                ctx.link().send_future(async move {
                    let body = CreateProjectBody {
                        title,
                        description,
                        due_date,
                    };

                    match post_request_struct::<CreateProjectBody, ContributionListItem>(
                        "http://localhost:8001/projects",
                        body,
                    )
                    .await
                    {
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
                    NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%S%.3f").unwrap();
                false
            }
            Msg::CreateProjectSuccess(new_item) => {
                match &mut self.contributions {
                    Some(contributions) => contributions.push(new_item),
                    None => self.contributions = Some(vec![new_item]),
                }
                true
            }
            Msg::CreateProjectFail(error) => {
                log_fetch_error(error);
                alert("Konnte Projekt nicht erstellen! Details ggfs. siehe Konsole.");
                false
            }
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
