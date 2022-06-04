use yew::{function_component, html, Callback, Component, Properties};

use gloo_console::error;

use crate::{
    service::submission::{delete_submission, submission_download_url, Submission},
    utilities::requests::fetch::FetchError,
};

pub struct SubmissionList {
    selected_submission: Option<i32>,
    selected_delete: Option<Submission>,
}

#[derive(PartialEq, Properties)]
pub struct SubmissionListProperties {
    pub submissions: Vec<Submission>,
    pub submission_delete: Callback<i32>,
}

pub enum DeleteMessage {
    ListItemButtonClick(Submission),
    AcceptClick,
    AbortClick,
    Success(i32),
    Fail(FetchError),
}

pub enum Msg {
    SelectOrUnselect(i32),
    Delete(DeleteMessage),
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
                                        { &submission.note }
                                    </td>
                                    <td>
                                        { format!("{:?}", &submission.creator_section) }
                                    </td>
                                    <td>
                                        { &submission.creator_name }
                                    </td>
                                    <td>
                                        <button class="btn btn-sm btn-outline-danger" onclick={ctx.link().callback(move |_| Msg::SelectOrUnselect(index as i32))}>{"Details"}</button>
                                    </td>
                                    <td>
                                    <a href={ submission_download_url(submission.id) } download="true">
                                        <button class="btn btn-sm btn-outline-danger">{"Herunterladen"}</button>
                                    </a>
                                    </td>
                                    <td>
                                        <button class="btn btn-sm btn-danger" onclick={ctx.link().callback(move |_| Msg::Delete(DeleteMessage::ListItemButtonClick(submission_clone.clone())))} data-bs-toggle="modal" data-bs-target="#modalSubmissionDelete">{"Löschen"}</button>
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
            <div class="modal fade" id="modalSubmissionDelete" data-bs-backdrop="static" tabindex="-1">
                <div class="modal-dialog">
                    <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{ "Modal title" }</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <p>{"Die Abgabe "}
                        if let Some(s) = &self.selected_delete {
                            { &s.file_name }
                        }
                        { " wird unwiederruflich gelöscht." }</p>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal"  onclick={ ctx.link().callback(|_| Msg::Delete(DeleteMessage::AbortClick)) }>{ "Close" }</button>
                        <button type="button" class="btn btn-primary" onclick={ ctx.link().callback(|_| Msg::Delete(DeleteMessage::AcceptClick)) }>{ "Save changes" }</button>
                    </div>
                    </div>
                </div>
            </div>
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
                    false
                }
                DeleteMessage::AcceptClick => {
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
                    false
                }
                DeleteMessage::AbortClick => {
                    self.selected_delete = None;
                    false
                }
                DeleteMessage::Success(id) => {
                    ctx.props().submission_delete.emit(id);
                    false
                }
                DeleteMessage::Fail(error) => {
                    error!("Abgabe konnte nicht gelöscht werden.");
                    error!(format!("Fetch error while deleting: {:?}", error));
                    self.selected_submission = None;
                    false
                }
            },
        }
    }
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
