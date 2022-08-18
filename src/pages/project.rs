use crate::{
    components::{
        iframe::IFrame,
        jwt_context::get_token_data,
        material::Material,
        submission::{
            list::SubmissionList, InputSubmissionKind, InputSubmissionNote, InputSubmissionSection,
        },
        upload::Upload,
        PlaceholderOrContent,
    },
    service::{
        project::{all_submissions_link, project_data, submission_upload_url, ProjectTo},
        submission::{
            submissions_by_project, submissions_by_project_and_user, Submission, SubmissionKind,
        },
    },
    utilities::requests::fetch::FetchError,
};

use wasm_bindgen::UnwrapThrowExt;

use yew::{html, Component, Properties};

use gloo_console::error;
use gloo_dialogs::alert;

pub enum Msg {
    MetadataLoaded(ProjectTo),
    MetadataLoadError(FetchError),
    AllSubmissionsLoaded(Vec<Submission>),
    MySubmissionsLoaded(Vec<Submission>),
    SubmissionsLoadError(FetchError),
    SubmissionsAdminLoadError(FetchError),
    SubmissionDeleted(i32),
    SubmissionUpdated(Submission),
    SubmissionUploaded(String),
    SubmissionUploadError(String),
}

pub struct ProjectComponent {
    project_data: Option<ProjectTo>,
    all_submissions: Option<Vec<Submission>>,
    my_submissions: Vec<Submission>,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ProjectProperties {
    pub id: i32,
}

impl Component for ProjectComponent {
    type Message = Msg;

    type Properties = ProjectProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            project_data: None,
            all_submissions: None,
            my_submissions: Vec::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let my_section = get_token_data().unwrap_throw().section;
        match &self.project_data {
            Some(metadata) => html! {
                <>
                <div class="row mt-2">
                    <div class="col">
                        <h1>{ &metadata.title }</h1>
                        <IFrame content={metadata.description.clone()}/>
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <a href="/"> <button type="button" class="btn btn-outline-danger"> { "zurück" } </button></a>
                    </div>

                    <div class="col">
                        <a href={ all_submissions_link(metadata.id) }>
                        <button class="btn btn-danger">{ "Abgaben downloaden" } </button>
                        </a>
                    </div>
                </div>

                <Material id={ctx.props().id} project_owner={metadata.creator}/>

                <div class="row mt-2">
                    <div class="col">
                        <h2>{ "Neue Datei hochladen" }</h2>
                        <form id="inputSubmissionUpload" class="" name="formMaterial" enctype="multipart/form-data">
                            <div class="row">
                                <div class="col">
                                    <InputSubmissionNote id={ "inputContentTitle".to_string() } placeholder_or_content={PlaceholderOrContent::Placeholder("z.B. Takt 15 bitte rausschneiden...".into())}/>
                                </div>
                                <div class="col-auto">
                                    <InputSubmissionSection id={ "selectUpdatedSection".to_string() } selected={ crate::service::submission::Section::from(my_section) }/>
                                </div>
                                <div class="col-auto">
                                    <InputSubmissionKind id={ "selectSubmissionKind".to_string() } selected={ SubmissionKind::Other }/>
                                </div>
                            </div>
                            <div class="row mt-2">
                                <div class="col">
                                    <Upload form_id="inputSubmissionUpload" field_name="file" target_url={ submission_upload_url(ctx.props().id) } multiple=true success_callback={ ctx.link().callback(Msg::SubmissionUploaded) } failure_callback={ ctx.link().callback(Msg::SubmissionUploadError) } />
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <h4>{ "Meine Abgaben" }</h4>
                    </div>
                </div>
                <div class="row mt-2">
                    <div class="col">
                        <SubmissionList id="list1" submissions={ self.my_submissions.clone() } submission_delete={ ctx.link().callback(Msg::SubmissionDeleted) } submission_update={ ctx.link().callback(Msg::SubmissionUpdated) } />
                    </div>
                </div>
                if let Some(all_submissions) = &self.all_submissions {
                    <div class="row mt-2">
                        <div class="col">
                            <h4>{ "Alle Abgaben" }</h4>
                        </div>
                    </div>
                    <div class="row mt-2">
                        <div class="col">
                            <SubmissionList id="list2" submissions={ all_submissions.clone() } submission_delete={ ctx.link().callback(Msg::SubmissionDeleted) } submission_update={ ctx.link().callback(Msg::SubmissionUpdated) }/>
                        </div>
                    </div>
                }
                </>
            },
            None => html! {
                <h2>{ "Daten werden geladen" }</h2>
            },
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            load_data(ctx);

            let project_id = ctx.props().id;
            ctx.link().send_future(async move {
                let submissions = submissions_by_project(project_id).await;

                match submissions {
                    Ok(contributions) => Msg::AllSubmissionsLoaded(contributions),
                    Err(error) => Msg::SubmissionsAdminLoadError(error),
                }
            });

            ctx.link().send_future(async move {
                let submissions = submissions_by_project_and_user(project_id, 1).await;

                match submissions {
                    Ok(contributions) => Msg::MySubmissionsLoaded(contributions),
                    Err(error) => Msg::SubmissionsLoadError(error),
                }
            });
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MetadataLoaded(metadata) => {
                self.project_data = Some(metadata);
                true
            }
            Msg::MetadataLoadError(error) => {
                alert("Could not download data!");
                match error {
                    FetchError::JsError(js_error) => error!(js_error),
                    FetchError::WrongContentType => {
                        error!("Content type of fetched object was wrong!")
                    }
                    FetchError::SerdeError(serde_error) => {
                        error!("type could not be parsed!");
                        error!(serde_error.to_string());
                    }
                    FetchError::StatusCode(status) => {
                        error!("Got status {} while downloading metadata", status);
                    }
                }
                true
            }

            Msg::MySubmissionsLoaded(submissions) => {
                self.my_submissions = submissions;
                self.sort_submissions();
                true
            }
            Msg::AllSubmissionsLoaded(submissions) => {
                self.all_submissions = Some(submissions);
                self.sort_submissions();
                true
            }
            Msg::SubmissionsLoadError(error) => {
                if let FetchError::StatusCode(status) = error {
                    if status == 401 {
                        return false;
                    }
                }
                gloo_console::error!(format!("{:?}", error));
                alert("Could not get submissions! For more info see the console log.");

                false
            }
            Msg::SubmissionsAdminLoadError(error) => {
                gloo_console::debug!(format!("could not get admin submissions {:?}", error));
                false
            }
            Msg::SubmissionDeleted(id) => {
                self.my_submissions.retain(|submission| submission.id != id);
                if let Some(all_submissions) = &mut self.all_submissions {
                    all_submissions.retain(|submission| submission.id != id);
                }
                true
            }
            Msg::SubmissionUploaded(text) => {
                let submission: Submission = serde_json::from_str(&text).unwrap_throw();
                if let Some(submissions) = &mut self.all_submissions {
                    submissions.push(submission.clone());
                }
                self.my_submissions.push(submission);
                self.sort_submissions();
                true
            }
            Msg::SubmissionUploadError(response_text) => {
                alert("Beim hochladen ist ein Fehler aufgetreten! Versuche es erneut und wende dich dann an den/die Administrator*in");
                error!(format!(
                    "Error while uploading submission! Response text: {}",
                    response_text
                ));
                false
            }
            Msg::SubmissionUpdated(submission) => {
                let user = get_token_data().unwrap_throw();
                if submission.creator == user.user_id {
                    self.my_submissions.retain(|x| x.id != submission.id);
                    self.my_submissions.push(submission.clone());
                }

                if let Some(submissions) = &mut self.all_submissions {
                    submissions.retain(|x| x.id != submission.id);
                    submissions.push(submission.clone());
                }

                self.sort_submissions();

                true
            }
        }
    }
}

impl ProjectComponent {
    fn sort_submissions(&mut self) {
        if let Some(submissions) = &mut self.all_submissions {
            submissions.sort_by(|a, b| {
                a.creator_section
                    .cmp(&b.creator_section)
                    .then(a.creator_name.cmp(&b.creator_name))
            });
        }

        self.my_submissions
            .sort_by(|a, b| a.file_name.cmp(&b.file_name))
    }
}

fn load_data(ctx: &yew::Context<ProjectComponent>) {
    let project_id = ctx.props().id;
    ctx.link().send_future(async move {
        match project_data(project_id).await {
            Ok(metadata) => Msg::MetadataLoaded(metadata),
            Err(error) => Msg::MetadataLoadError(error),
        }
    })
}
