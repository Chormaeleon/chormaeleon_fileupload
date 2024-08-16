use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use yew::{html, ToHtml};

use crate::{
    components::jwt_context::{self},
    utilities::requests::fetch::{
        delete_request, get_request_struct, post_request_struct, FetchError,
    },
};

use super::backend;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub project_id: i64,
    pub note: String,
    pub file_name: String,
    pub file_technical_name: String,
    pub submitter: i64,
    pub creator: i64,
    pub creator_name: String,
    pub creator_section: Section,
    pub upload_at: PrimitiveDateTime,
    pub kind: SubmissionKind,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Section {
    Soprano1,
    Soprano2,
    Alto1,
    Alto2,
    Tenor1,
    Tenor2,
    Bass1,
    Bass2,
    Conductor,
    Instrument,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Section::Soprano1 => "Sopran 1",
            Section::Alto1 => "Alt 1",
            Section::Tenor1 => "Tenor 1",
            Section::Bass1 => "Bass 1",
            Section::Soprano2 => "Sopran 2",
            Section::Alto2 => "Alt 2",
            Section::Tenor2 => "Tenor 2",
            Section::Bass2 => "Bass 2",
            Section::Conductor => "Dirigent",
            Section::Instrument => "Instrument",
        };

        write!(f, "{}", content)
    }
}

impl ToHtml for Section {
    fn to_html(&self) -> yew::Html {
        html!(self)
    }
}

impl TryFrom<&str> for Section {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value {
            "Soprano1" => Self::Soprano1,
            "Alto1" => Self::Alto1,
            "Tenor1" => Self::Tenor1,
            "Bass1" => Self::Bass1,
            "Soprano2" => Self::Soprano2,
            "Alto2" => Self::Alto2,
            "Tenor2" => Self::Tenor2,
            "Bass2" => Self::Bass2,
            "Conductor" => Self::Conductor,
            "Instrument" => Self::Instrument,
            _ => return Err(()),
        };

        Ok(result)
    }
}

impl From<jwt_context::Section> for Section {
    fn from(section: jwt_context::Section) -> Self {
        match section {
            jwt_context::Section::Soprano1 => Self::Soprano1,
            jwt_context::Section::Alto1 => Self::Alto1,
            jwt_context::Section::Tenor1 => Self::Tenor1,
            jwt_context::Section::Bass1 => Self::Bass1,
            jwt_context::Section::Soprano2 => Self::Soprano2,
            jwt_context::Section::Alto2 => Self::Alto2,
            jwt_context::Section::Tenor2 => Self::Tenor2,
            jwt_context::Section::Bass2 => Self::Bass2,
            jwt_context::Section::Conductor => Self::Conductor,
            jwt_context::Section::Instrument => Self::Instrument,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SubmissionKind {
    Audio,
    Video,
    Document,
    Other,
}

impl Display for SubmissionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            SubmissionKind::Audio => "Audio",
            SubmissionKind::Video => "Video",
            SubmissionKind::Document => "Dokument",
            SubmissionKind::Other => "Sonstiges",
        };

        write!(f, "{}", content)
    }
}

impl ToHtml for SubmissionKind {
    fn to_html(&self) -> yew::Html {
        html! {
            self
        }
    }
}

/// mainly for matching file extensions
impl From<&str> for SubmissionKind {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "wav" | "mp3" | "flac" | "wma" | "aac" | "ogg" | "audio" => SubmissionKind::Audio,
            "mp4" | "avi" | "mov" | "flv" | "f4v" | "swf" | "wmv" | "avchd" | "mkv" | "webm"
            | "video" => SubmissionKind::Video,
            "pdf" | "document" => SubmissionKind::Document,
            _ => SubmissionKind::Other,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSubmission {
    pub note: String,
    pub section: Section,
    pub kind: SubmissionKind,
    pub creator_name: String,
}

pub async fn submissions_by_project(project_id: i64) -> Result<Vec<Submission>, FetchError> {
    let backend_url = backend();
    get_request_struct::<Vec<Submission>>(&format!(
        "{backend_url}/projects/{}/submissions",
        project_id
    ))
    .await
}

pub async fn submissions_by_project_and_user(
    project_id: i64,
    user_id: i64,
) -> Result<Vec<Submission>, FetchError> {
    let backend_url = backend();
    get_request_struct::<Vec<Submission>>(&format!(
        "{backend_url}/projects/{project_id}/submissions/{user_id}"
    ))
    .await
}

pub fn submission_download_url(submission_id: i64) -> String {
    let backend_url = backend();
    format!("{backend_url}/submissions/{submission_id}")
}

pub fn submission_stream_url(project_id: i64, file_technical_name: &str) -> String {
    let backend_url = backend();
    format!("{backend_url}/submissions/stream/{project_id}/{file_technical_name}")
}

pub async fn update_submission(
    submission_id: i64,
    update_data: UpdateSubmission,
) -> Result<Submission, FetchError> {
    let backend_url = backend();
    post_request_struct(
        &format!("{backend_url}/submissions/{submission_id}"),
        update_data,
    )
    .await
}

pub async fn delete_submission(submission_id: i64) -> Result<(), FetchError> {
    let backend_url = backend();
    delete_request(&format!("{backend_url}/submissions/{submission_id}")).await
}
