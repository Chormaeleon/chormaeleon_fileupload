use gloo_dialogs::alert;
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Properties};

use gloo_console::error;

use crate::{
    components::{
        delete_modal::DeleteModal,
        submission::{
            details::SubmissionDetails,
            update::{SubmissionUpdate, SubmissionUpdateData},
        },
    },
    service::submission::{
        delete_submission, submission_download_url, update_submission, Submission, UpdateSubmission,
    },
    utilities::requests::fetch::FetchError,
};

pub struct SubmissionList {
    selected_submission: Option<i64>,
    selected_delete: Option<Submission>,
    selected_update: Option<Submission>,
}

#[derive(PartialEq, Properties)]
pub struct SubmissionListProperties {
    pub submissions: Vec<Submission>,
    pub submission_delete: Callback<i64>,
    pub submission_update: Callback<Submission>,
    pub id: String,
}

pub enum UpdateMessage {
    Init(Submission),
    Abort(MouseEvent),
    Submit(SubmissionUpdateData),
    Success(Submission),
    Error(FetchError),
}

pub enum DeleteMessage {
    ListItemButtonClick(Submission),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success(i64),
    Fail(FetchError),
}

pub enum Msg {
    SelectOrUnselect(i64),
    Delete(DeleteMessage),
    Update(UpdateMessage),
}

impl Component for SubmissionList {
    type Properties = SubmissionListProperties;
    type Message = Msg;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            selected_submission: None,
            selected_delete: None,
            selected_update: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
            <div class="table-responsive">
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th>
                                { "Name" }
                            </th>
                            <th>
                                { "Art" }
                            </th>
                            <th>
                                { "Kommentar" }
                            </th>
                            <th>
                                { "Stimme" }
                            </th>
                            <th>
                                { "Autor*in" }
                            </th>
                            <th>
                                { "Details" }
                            </th>
                            <th>
                                { "Herunterladen" }
                            </th>
                            <th>
                                { "Ändern" }
                            </th>
                            <th>
                                { "Löschen" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        if ctx.props().submissions.is_empty() {
                            html!{
                                <td>{ "Keine Abgaben gefunden" }</td>
                            }
                        } else {
                            html! {
                                <>
                                {
                                    for ctx.props().submissions.iter().enumerate().map(|(index, submission)| {
                                        let submission_clone = submission.clone();
                                        let submission_clone_2 = submission.clone();
                                        html! {
                                            <>
                                            <tr>
                                                <td>
                                                    { &submission.file_name }
                                                </td>
                                                <td>
                                                    { &submission.kind }
                                                </td>
                                                <td>
                                                    { &submission.note }
                                                </td>
                                                <td>
                                                    { submission.creator_section }
                                                </td>
                                                <td>
                                                    { &submission.creator_name }
                                                </td>
                                                <td>
                                                    <button class="btn btn-sm btn-outline-danger" onclick={ ctx.link().callback(move |_| Msg::SelectOrUnselect(index as i64)) }>{"Details"}</button>
                                                </td>
                                                <td>
                                                    <a href={ submission_download_url(submission.id) } download="true" target="_blank">
                                                        <button class="btn btn-sm btn-outline-danger">
                                                            {"Herunterladen"}
                                                        </button>
                                                    </a>
                                                </td>
                                                <td>
                                                    <button
                                                        class="btn btn-sm btn-outline-danger"
                                                        onclick={ ctx.link().callback(move |_| Msg::Update(UpdateMessage::Init(submission_clone.clone()))) }
                                                        data-bs-toggle="modal"
                                                        data-bs-target={ format!("#{}", update_modal_id(&ctx.props().id)) }>
                                                            { "Ändern" }
                                                    </button>
                                                </td>
                                                <td>
                                                    <button
                                                        class="btn btn-sm btn-danger"
                                                        onclick={ ctx.link().callback(move |_| Msg::Delete(DeleteMessage::ListItemButtonClick(submission_clone_2.clone()))) }
                                                        data-bs-toggle="modal"
                                                        data-bs-target={ format!("#{}", delete_modal_id(&ctx.props().id)) }>
                                                            { "Löschen" }
                                                    </button>
                                                </td>
                                            </tr>

                                            if let Some(selected_index) = self.selected_submission  {
                                                if index as i64 == selected_index {
                                                    <tr>
                                                        <td colspan="10">
                                                            <SubmissionDetails submission={ submission.clone() } />
                                                        </td>
                                                    </tr>
                                                }
                                            }
                                            </>
                                        }
                                    })
                                }
                                </>
                            }
                        }
                    }
                    </tbody>
                </table>
            </div>
            <SubmissionUpdate
                id={ update_modal_id(&ctx.props().id) }
                submission={ self.selected_update.clone() }
                on_abort={ ctx.link().callback(|x| Msg::Update(UpdateMessage::Abort(x))) }
                on_submit={ ctx.link().callback(|x| Msg::Update(UpdateMessage::Submit(x))) }
            />

            <DeleteModal
                    title={"Projekt löschen".to_string() }
                    id={ delete_modal_id(&ctx.props().id) }
                    on_cancel={ ctx.link().callback(|x| Msg::Delete(DeleteMessage::AbortClick(x))) }
                    on_confirm={ ctx.link().callback(|x| Msg::Delete(DeleteMessage::AcceptClick(x))) }
            >
                <>
                    <p>
                        { "Warnung! Kann " }
                            <b>
                            { "nicht rückgängig " }
                            </b>
                        { "gemacht werden!" }
                    </p>
                    <p>
                        { "Die Abgabe " }
                        if let Some(submission) = &self.selected_delete {
                            <i> { &submission.file_name } </i>
                        }
                        else {
                            <h2> { "Fehler! Keine Abgabe ausgewählt!" } </h2>
                        }
                        { " wirklich löschen?" }
                    </p>
                </>
            </DeleteModal>
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectOrUnselect(index) => {
                if let Some(id) = self.selected_submission.take() {
                    if id == index {
                        return true;
                    }
                }

                self.selected_submission = Some(index);

                true
            }
            Msg::Delete(delete_message) => match delete_message {
                DeleteMessage::ListItemButtonClick(submission) => {
                    self.selected_delete = Some(submission);
                    true
                }
                DeleteMessage::AcceptClick(_) => {
                    match &self.selected_delete {
                        Some(s) => {
                            let id = s.id;
                            ctx.link().send_future(async move {
                                let result = delete_submission(id).await;
                                match result {
                                    Ok(()) => Msg::Delete(DeleteMessage::Success(id)),
                                    Err(error) => Msg::Delete(DeleteMessage::Fail(error)),
                                }
                            });
                        }
                        None => {
                            error!("Tried to confirm delete without selecting item!");
                        }
                    }
                    true
                }
                DeleteMessage::AbortClick(_) => {
                    self.selected_delete = None;
                    true
                }
                DeleteMessage::Success(id) => {
                    ctx.props().submission_delete.emit(id);
                    self.selected_delete = None;
                    false
                }
                DeleteMessage::Fail(error) => {
                    error!(format!(
                        "Fetch error while deleting submission: {:?}",
                        error
                    ));
                    alert("Fehler beim Löschen der Abgabe.");
                    self.selected_submission = None;
                    true
                }
            },
            Msg::Update(message) => match message {
                UpdateMessage::Init(submission) => {
                    self.selected_update = Some(submission);
                    true
                }
                UpdateMessage::Abort(_) => {
                    self.selected_delete = None;
                    true
                }
                UpdateMessage::Submit(data) => {
                    ctx.link().send_future(async move {
                        match update_submission(
                            data.id,
                            UpdateSubmission {
                                note: data.note,
                                section: data.section,
                                kind: data.kind,
                            },
                        )
                        .await
                        {
                            Ok(submission) => Msg::Update(UpdateMessage::Success(submission)),
                            Err(error) => Msg::Update(UpdateMessage::Error(error)),
                        }
                    });

                    self.selected_update = None;

                    true
                }
                UpdateMessage::Success(submission) => {
                    ctx.props().submission_update.emit(submission);
                    false
                }
                UpdateMessage::Error(error) => {
                    alert("Daten der Abgabe konnten nicht geändert werden. Details siehe Konsole.");
                    error!(format!("{}", error));
                    false
                }
            },
        }
    }
}

fn delete_modal_id(owner_id: &str) -> String {
    format!("modalSubmissionDelete{owner_id}")
}

fn update_modal_id(owner_id: &str) -> String {
    format!("modalSubmissionUpdate{owner_id}")
}
