use gloo_dialogs::alert;
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Html, Properties};

use gloo_console::error;

use crate::{
    components::{
        modal::Modal,
        submission::{InputSubmissionKind, InputSubmissionNote, InputSubmissionSection},
    },
    service::submission::{Section, Submission, SubmissionKind},
};

pub const MODAL_UPDATE_SUBMISSION: &str = "ModalUpdateSubmission";

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SubmissionUpdateProperties {
    pub submission: Option<Submission>,
    pub on_abort: Callback<MouseEvent>,
    pub on_submit: Callback<SubmissionUpdateData>,
}

pub struct SubmissionUpdate {
    data: Option<SubmissionUpdateData>,
}

#[derive(Clone, Debug)]
pub struct SubmissionUpdateData {
    pub id: i32,
    pub note: String,
    pub section: Section,
    pub kind: SubmissionKind,
}

pub enum UpdateMsg {
    Note(String),
    Section(Result<Section, ()>),
    Kind(SubmissionKind),
    Submit(MouseEvent),
}

impl Component for SubmissionUpdate {
    type Message = UpdateMsg;

    type Properties = SubmissionUpdateProperties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            data: Self::submission_changed(ctx),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let data = match &mut self.data {
            Some(data) => data,
            None => {
                data_not_initialized_warning();
                return false;
            }
        };

        match msg {
            UpdateMsg::Note(note) => {
                data.note = note;
                false
            }
            UpdateMsg::Section(section) => {
                let section = match section {
                    Ok(s) => s,
                    Err(_) => {
                        enum_implement_warning("section");
                        return false;
                    }
                };

                data.section = section;
                false
            }
            UpdateMsg::Kind(kind) => {
                data.kind = kind;
                false
            }
            UpdateMsg::Submit(_) => {
                ctx.props().on_submit.emit(data.clone());
                self.data = None;
                false
            }
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        self.data = Self::submission_changed(ctx);
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let submission = &ctx.props().submission;

        html! {
            <Modal
                title={ "Abgabe anpassen".to_string() }
                id= { MODAL_UPDATE_SUBMISSION.to_string() }
                actions = { vec![
                    ("Abbrechen".to_string(), "btn btn-secondary".to_string(), ctx.props().on_abort.clone()),
                    ("Erstellen".to_string(), "btn btn-danger".to_string(), ctx.link().callback(UpdateMsg::Submit))
                    ]
                }
            >
            {
                match submission {
                    Some(submission) => {
                        html!{
                        <form>
                            <div class="row">
                                    <div class="col">
                                        <InputSubmissionNote id={ "inputUpdatedSubmissionNote".to_string() } value={ submission.note.clone() } on_input={ctx.link().callback(UpdateMsg::Note)}/>
                                    </div>
                                    <div class="col-auto">
                                        <InputSubmissionSection id={ "selectUpdatedSection".to_string() } selected={ submission.creator_section } on_input={ctx.link().callback(UpdateMsg::Section)}/>
                                    </div>
                                    <div class="col-auto">
                                        <InputSubmissionKind id={ "selectUpdatedSubmissionKind".to_string() } selected={ submission.kind } on_input={ctx.link().callback(UpdateMsg::Kind)}/>
                                    </div>
                                </div>
                        </form>
                        }
                    }

                    None => html!{{ "Fehler! Keine Abgabe ausgewählt!" }}
                }

            }
            </Modal>
        }
    }
}

fn data_not_initialized_warning() {
    alert("Keine Abgabe zur Änderung ausgewählt. Bitte das Fenster schließen und es erneut versuchen!");
}

fn enum_implement_warning(field: &str) {
    alert("Bitte merke Dir die Schritte, die du ausgeführt hast. Ein Fehler ist aufgetreten, wende dich an den/die Administrator*in");

    error!("Select option could not be parsed: {}", field);
}

impl SubmissionUpdate {
    fn submission_changed(ctx: &yew::Context<Self>) -> Option<SubmissionUpdateData> {
        ctx.props()
            .submission
            .as_ref()
            .map(|submission| SubmissionUpdateData {
                id: submission.id,
                note: submission.note.clone(),
                section: submission.creator_section,
                kind: submission.kind,
            })
    }
}
