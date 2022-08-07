use gloo_dialogs::alert;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Component, Properties};

use gloo_console::error;

use crate::{
    components::delete_modal::DeleteModal,
    service::submission::{
        delete_submission, get_submission_download_key, submission_download_url, Submission,
    },
    utilities::{download_from_link, requests::fetch::FetchError},
};

pub struct SubmissionList {
    selected_submission: Option<i32>,
    selected_delete: Option<Submission>,
}

#[derive(PartialEq, Properties)]
pub struct SubmissionListProperties {
    pub submissions: Vec<Submission>,
    pub submission_delete: Callback<i32>,
    pub id: String,
}

pub enum DeleteMessage {
    ListItemButtonClick(Submission),
    AcceptClick(MouseEvent),
    AbortClick(MouseEvent),
    Success(i32),
    Fail(FetchError),
}

pub enum Msg {
    SelectOrUnselect(i32),
    Delete(DeleteMessage),
    DownloadClicked(i32),
    DownloadKey(i32, String),
    DownloadKeyError(FetchError),
}

impl Component for SubmissionList {
    type Properties = SubmissionListProperties;
    type Message = Msg;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            selected_submission: None,
            selected_delete: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
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
                            { "Löschen" }
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for ctx.props().submissions.iter().enumerate().map(|(index, submission)| {
                            let submission_clone = submission.clone();
                            html!{
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
                                        { format!("{:?}", &submission.creator_section) }
                                    </td>
                                    <td>
                                        { &submission.creator_name }
                                    </td>
                                    <td>
                                        <button class="btn btn-sm btn-outline-danger" onclick={ ctx.link().callback(move |_| Msg::SelectOrUnselect(index as i32)) }>{"Details"}</button>
                                    </td>
                                    <td>
                                        <button class="btn btn-sm btn-outline-danger" onclick={ ctx.link().callback(move |_| Msg::DownloadClicked(submission_clone.id)) }>{"Herunterladen"}</button>
                                    </td>
                                    <td>
                                        <button class="btn btn-sm btn-danger" onclick={ ctx.link().callback(move |_| Msg::Delete(DeleteMessage::ListItemButtonClick(submission_clone.clone()))) } data-bs-toggle="modal" data-bs-target={ format!("#{}", calc_id(&ctx.props().id)) }>{ "Löschen" }</button>
                                    </td>

                                </tr>
                                if let Some(selected_index) = self.selected_submission  {
                                    if index as i32 == selected_index {
                                        <tr>
                                            <td colspan=7>
                                                <SubmissionDetails submission={ submission.clone() }/>
                                            </td>
                                        </tr>
                                    }
                                }
                                </>
                            }
                        })
                    }
                </tbody>
            </table>
            <DeleteModal
                    title={"Projekt löschen".to_string() }
                    id={ calc_id(&ctx.props().id) }
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
                    false
                }
                DeleteMessage::Fail(error) => {
                    error!("Abgabe konnte nicht gelöscht werden.");
                    error!(format!("Fetch error while deleting: {:?}", error));
                    self.selected_submission = None;
                    true
                }
            },
            Msg::DownloadClicked(id) => {
                ctx.link().send_future(async move {
                    match get_submission_download_key(id).await {
                        Ok(key) => Msg::DownloadKey(id, key),
                        Err(error) => Msg::DownloadKeyError(error),
                    }
                });
                false
            }
            Msg::DownloadKey(submission_id, key) => {
                download_from_link(&submission_download_url(submission_id, key));
                false
            }
            Msg::DownloadKeyError(error) => {
                error!(format!(
                    "Could not get download key for submission. Error: {:?}",
                    error
                ));

                alert("Download konnte nicht beendet werden. Fehler siehe Konsole. Bitte es erneut versuchen und sich dann an den/die Administrator*in wenden.");
                false
            }
        }
    }
}

fn calc_id(own_id: &str) -> String {
    format!("modalSubmissionDelete{own_id}")
}

#[derive(PartialEq, Properties)]
pub struct SubmissionProperty {
    submission: Submission,
}

#[function_component(SubmissionDetails)]
fn submission_details(s: &SubmissionProperty) -> Html {
    let submission = &s.submission;
    html!(<>
        <h4>
            { &submission.note }
        </h4>
        <div class="row">
            <div class="col">
                <b>{ "Dateiname: " }</b>
                <i>{ &submission.file_name } </i>
            </div>
            <div class="col">
                <b>{ "Id: " }</b>
                { submission.id }
            </div>
            <div class="col">
                <b>{ "Autor (Id)" }</b>
                { submission.creator }
            </div>
        </div>
        </>)
}
