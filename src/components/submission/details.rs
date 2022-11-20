use yew::{function_component, html, Properties};

use crate::{
    service::submission::{submission_stream_url, Submission},
    utilities::date::format_datetime_human_readable_seconds,
};

#[derive(Eq, PartialEq, Properties)]
pub struct SubmissionProperties {
    pub submission: Submission,
}

#[function_component(SubmissionDetails)]
pub fn submission_details(s: &SubmissionProperties) -> Html {
    let submission = &s.submission;
    html!(<>
        
        <table class="table">
            <tbody>
            <caption>
            { &submission.note }
            </caption>
            <tr>
                <td>
                    <b>{ "Dateiname:" }</b> 
                </td>
                <td> 
                    <i>{ &submission.file_name } </i>
                </td>
                <td>
                    <b>{ "Id: " }</b> 
                </td>
                <td>
                    { submission.id }
                </td>
                <td>
                    <b>{ "Hochgeladen: " }</b> 
                </td>
                <td>
                    { format_datetime_human_readable_seconds(&submission.upload_at) }
                </td>
            </tr>
            <tr>
                <td>
                    <b>{ "Autor (Id): " }</b>   
                </td>
                <td>
                    { submission.creator }
                </td>
                <td>
                    <b>{ "Autor (Name): " }</b>
                </td>
                <td>
                    { &submission.creator_name }
                </td>
                <td>
                    <b>{ "Eingereicht von: " }</b>   
                </td>
                <td>
                    { submission.submitter }     
                </td>
            </tr>
        </tbody>
        </table>
        <div class="row mt-2">
            <div class="col">
                {
                    match submission.kind {
                        crate::service::submission::SubmissionKind::Audio => html!{
                            <>
                            <h5> { "Vorschau: " }</h5>
                            <audio controls=true src={ submission_stream_url(submission.project_id, &submission.file_technical_name) }></audio>
                            </>
                        },
                        crate::service::submission::SubmissionKind::Video => html!{
                            <>
                            <h5> { "Vorschau: " }</h5>
                            <div class="ratio ratio-16x9">
                                <video controls=true>
                                    <source src={ submission_stream_url(submission.project_id, &submission.file_technical_name) }/>
                                </video>
                            </div>
                            </>
                        },
                        crate::service::submission::SubmissionKind::Other => html!{
                            <p> { "Für \"Sonstiges\" kann keine Vorschau erstellt werden. Passe gegebenfalls die Art der Abgabe über die Schaltfläche \"Ändern\" an!" } </p>
                        },
                    }
                }
            </div>
        </div>
        </>
    )
}
