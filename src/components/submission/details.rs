use yew::{function_component, html, Properties};

use crate::service::submission::{submission_stream_url, Submission};

#[derive(Eq, PartialEq, Properties)]
pub struct SubmissionProperties {
    pub submission: Submission,
}

#[function_component(SubmissionDetails)]
pub fn submission_details(s: &SubmissionProperties) -> Html {
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
        <div class="row">
            <div class="col">
                {
                    match submission.kind {
                        crate::service::submission::SubmissionKind::Audio => html!{
                            <audio controls=true src={ submission_stream_url(submission.project_id, &submission.file_technical_name) }></audio>

                        },
                        crate::service::submission::SubmissionKind::Video => html!{
                            <div class="ratio ratio-16x9">
                                <video controls=true>
                                    <source src={ submission_stream_url(submission.project_id, &submission.file_technical_name) }/>
                                </video>
                            </div>
                        },
                        crate::service::submission::SubmissionKind::Other => html!{},
                    }
                }
            </div>
        </div>
        </>
    )
}
