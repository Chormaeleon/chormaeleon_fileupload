use gloo_dialogs::alert;
use time::{macros::format_description, PrimitiveDateTime, Time};
use web_sys::{Event, InputEvent, MouseEvent};
use yew::{html, Callback, Component, Properties};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    components::modal::Modal,
    pages::home::{get_value_from_event, get_value_from_input_event},
    service::project::{create_project, update_project, ProjectTo},
    utilities::{date::now, requests::fetch::FetchError},
};

pub const MODAL_NEW_PROJECT: &str = "modalNewProject";
pub const MODAL_UPDATE_PROJECT: &str = "modalUpdateProject";

#[wasm_bindgen(module = "/js/custom/tinymce_content.js")]
extern "C" {

    #[wasm_bindgen]
    fn set_tinymce_content(editor: String, content: String);
    fn get_tinymce_content(editor: String) -> String;
}

pub struct ProjectUpdateModal {}

pub enum UpdateMessage {
    Success(ProjectTo),
    Fail(FetchError),
    ButtonClick(ModalResult),
}

#[derive(PartialEq, Properties)]
pub struct ProjectUpdateProperties {
    pub on_success: Callback<ProjectTo>,
    pub on_error: Callback<FetchError>,
    pub project: Option<ProjectTo>,
}

impl Component for ProjectUpdateModal {
    type Message = UpdateMessage;
    type Properties = ProjectUpdateProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UpdateMessage::Success(project) => {
                ctx.props().on_success.emit(project);
                false
            }
            UpdateMessage::Fail(error) => {
                ctx.props().on_error.emit(error);
                false
            }
            UpdateMessage::ButtonClick(result) => {
                if result.title.is_empty() {
                    alert("Titel fehlt!");
                    return false;
                }

                ctx.link().send_future(async move {
                    match update_project(
                        result.id.unwrap(),
                        result.title,
                        result.description,
                        result.due,
                    )
                    .await
                    {
                        Ok(result) => UpdateMessage::Success(result),
                        Err(error) => UpdateMessage::Fail(error),
                    }
                });

                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! (
            <ProjectEditModal on_submit={ctx.link().callback(UpdateMessage::ButtonClick)} project={ctx.props().project.clone()} id={ MODAL_UPDATE_PROJECT }/>
        )
    }
}

pub struct ProjectCreateModal {}

pub enum CreateMessage {
    Success(ProjectTo),
    Fail(FetchError),
    ButtonClick(ModalResult),
}

#[derive(PartialEq, Properties)]
pub struct ProjectCreateProperties {
    pub on_success: Callback<ProjectTo>,
    pub on_error: Callback<FetchError>,
}

impl Component for ProjectCreateModal {
    type Message = CreateMessage;
    type Properties = ProjectCreateProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CreateMessage::Success(project) => {
                ctx.props().on_success.emit(project);
                false
            }
            CreateMessage::Fail(error) => {
                ctx.props().on_error.emit(error);
                false
            }
            CreateMessage::ButtonClick(result) => {
                if result.title.is_empty() {
                    alert("Titel fehlt!");
                    return false;
                }

                ctx.link().send_future(async move {
                    match create_project(result.title, result.description, result.due).await {
                        Ok(result) => CreateMessage::Success(result),
                        Err(error) => CreateMessage::Fail(error),
                    }
                });

                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! (
            <ProjectEditModal on_submit={ctx.link().callback(CreateMessage::ButtonClick)} id={ MODAL_NEW_PROJECT }/>
        )
    }
}

pub struct ModalResult {
    id: Option<i64>,
    title: String,
    description: String,
    due: PrimitiveDateTime,
}

#[derive(Clone, PartialEq, Properties)]
struct ProjectEditModalProperties {
    #[prop_or_default]
    pub project: Option<ProjectTo>,
    pub on_submit: Callback<ModalResult>,
    pub id: String,
}

struct ProjectEditModal {
    id: Option<i64>,
    title: String,
    due: PrimitiveDateTime,
}

enum Msg {
    #[allow(dead_code)]
    AbortClick(MouseEvent),
    #[allow(dead_code)]
    CreateClick(MouseEvent),
    NameInput(InputEvent),
    DateInput(Event),
}

impl Component for ProjectEditModal {
    type Message = Msg;
    type Properties = ProjectEditModalProperties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        match &ctx.props().project {
            Some(project) => Self {
                id: Some(project.id),
                due: project.due,
                title: project.title.clone(),
            },
            None => Self {
                id: None,
                due: now().replace_time(Time::from_hms(23, 59, 59).unwrap()),
                title: "".to_string(),
            },
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NameInput(event) => {
                self.title = get_value_from_input_event(event);
                false
            }
            Msg::DateInput(event) => {
                let date_string = get_value_from_event(event);

                if date_string.is_empty() {
                    return false;
                }

                self.due = match PrimitiveDateTime::parse(
                    &date_string,
                    format_description!(
                        "[year]-[month]-[day]T[hour]:[minute]:[second]:[subsecond]"
                    ),
                ) {
                    Ok(date_time) => date_time,
                    Err(_e) => PrimitiveDateTime::parse(
                        &date_string,
                        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
                    )
                    .unwrap_or_else(|_| {
                        PrimitiveDateTime::parse(
                            &date_string,
                            format_description!("[year]-[month]-[day]T[hour]:[minute]"),
                        )
                        .unwrap_or_else(|error| {
                            panic!("Could not parse date {date_string}, error: {error}")
                        })
                    }),
                };
                false
            }
            Msg::AbortClick(_) => false,
            Msg::CreateClick(_) => {
                ctx.props().on_submit.emit(ModalResult {
                    id: self.id,
                    title: self.title.clone(),
                    description: get_tinymce_content(self.text_area_name(ctx)),
                    due: self.due,
                });

                false
            }
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(project) = &ctx.props().project {
            self.id = Some(project.id);
            self.due = project.due;
            self.title = project.title.clone();
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let text_area_name = self.text_area_name(ctx);

        let tinymce = format!(
            "tinymce.init({{
                selector: '#{text_area_name}',
            }});"
        );

        let (title, description, date) = match &ctx.props().project {
            Some(project) => (
                project.title.clone(),
                project.description.clone(),
                format_date_to_end_of_day(project.due),
            ),
            None => ("".to_string(), "".to_string(), end_of_today()),
        };

        if !description.is_empty() {
            set_tinymce_content(text_area_name.clone(), description);
        }

        html! (
            <Modal
                title={"Projekt erstellen".to_string() }
                id={ ctx.props().id.clone() }
                actions = { vec![
                    ("Abbrechen".to_string(), "btn btn-secondary".to_string(), ctx.link().callback(Msg::AbortClick)),
                    ("Erstellen".to_string(), "btn btn-danger".to_string(),  ctx.link().callback(Msg::CreateClick))
                    ]
                }
            >
            <>
                <script>
                {  tinymce }
                </script>

                <form id="createProjectForm" class="">
                    <div class="row">
                        <div class="col">
                            <label for="inputCreateProjectTitle">{ "Name des Projektes" }</label>
                            <input id="inputCreateProjectTitle" type="text" class="form-control" value={title} placeholder="Name des Projektes" oninput={ ctx.link().callback(Msg::NameInput) }/>
                        </div>
                        <div class="col">
                            <label for="inputCreateProjectDueDate">{ "Abgabedatum" }</label>
                            <input id="inputCreateProjectDueDate" type="datetime-local" class="form-control" value={date} onchange={ ctx.link().callback(Msg::DateInput) }/>
                        </div>
                    </div>
                    <div class="row mt-2">
                        <div class="col">
                            <label for={ text_area_name.clone() }>{ "Beschreibung" }</label>
                            <textarea id={ text_area_name }></textarea>
                        </div>
                    </div>

                </form>
            </>
            </Modal>
        )
    }
}

impl ProjectEditModal {
    fn text_area_name(&self, ctx: &yew::Context<Self>) -> String {
        format!("textarea{}", ctx.props().id)
    }
}

fn end_of_today() -> String {
    format_date_to_end_of_day(now())
}

fn format_date_to_end_of_day(date_time: PrimitiveDateTime) -> String {
    let format = format_description!("[year]-[month]-[day]T23:59");
    date_time.format(&format).unwrap()
}
