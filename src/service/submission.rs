use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    components::jwt_context::{self},
    utilities::requests::fetch::{
        delete_request, get_request_struct, post_request_struct, FetchError,
    },
};

use super::BACKEND_URL;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Submission {
    pub id: i32,
    pub project_id: i32,
    pub note: String,
    pub file_name: String,
    pub file_technical_name: String,
    pub submitter: i32,
    pub creator: i32,
    pub creator_name: String,
    pub creator_section: Section,
    pub upload_at: NaiveDateTime,
    pub kind: SubmissionKind,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Section {
    Soprano,
    Alto,
    Tenor,
    Bass,
    Conductor,
    Instrument,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Section::Soprano => "Sopran",
            Section::Alto => "Alt",
            Section::Tenor => "Tenor",
            Section::Bass => "Bass",
            Section::Conductor => "Dirigent",
            Section::Instrument => "Instrument",
        };

        write!(f, "{}", content)
    }
}

impl TryFrom<&str> for Section {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value {
            "Soprano" => Self::Soprano,
            "Alto" => Self::Alto,
            "Tenor" => Self::Tenor,
            "Bass" => Self::Bass,
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
            jwt_context::Section::Soprano => Self::Soprano,
            jwt_context::Section::Alto => Self::Alto,
            jwt_context::Section::Tenor => Self::Tenor,
            jwt_context::Section::Bass => Self::Bass,
            jwt_context::Section::Conductor => Self::Conductor,
            jwt_context::Section::Instrument => Self::Instrument,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SubmissionKind {
    Audio,
    Video,
    Other,
}

impl Display for SubmissionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            SubmissionKind::Audio => "Audio",
            SubmissionKind::Video => "Video",
            SubmissionKind::Other => "Sonstiges",
        };

        write!(f, "{}", content)
    }
}

/// mainly for matching file extensions
impl From<&str> for SubmissionKind {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "wav" | "mp3" | "flac" | "wma" | "aac" | "ogg" | "audio" => SubmissionKind::Audio,
            "mp4" | "avi" | "mov" | "flv" | "f4v" | "swf" | "wmv" | "avchd" | "mkv" | "webm"
            | "video" => SubmissionKind::Video,

            _ => SubmissionKind::Other,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSubmission {
    pub note: String,
    pub section: Section,
    pub kind: SubmissionKind,
}

pub async fn submissions_by_project(project_id: i32) -> Result<Vec<Submission>, FetchError> {
    get_request_struct::<Vec<Submission>>(&format!(
        "{BACKEND_URL}/projects/{}/submissions",
        project_id
    ))
    .await
}

pub async fn submissions_by_project_and_user(
    project_id: i32,
    user_id: i32,
) -> Result<Vec<Submission>, FetchError> {
    get_request_struct::<Vec<Submission>>(&format!(
        "{BACKEND_URL}/projects/{project_id}/submissions/{user_id}"
    ))
    .await
}

pub fn submission_download_url(submission_id: i32) -> String {
    format!("{BACKEND_URL}/submissions/{submission_id}")
}

pub async fn update_submission(
    submission_id: i32,
    update_data: UpdateSubmission,
) -> Result<Submission, FetchError> {
    post_request_struct(
        &format!("{BACKEND_URL}/submissions/{submission_id}"),
        update_data,
    )
    .await
}

pub async fn delete_submission(submission_id: i32) -> Result<(), FetchError> {
    delete_request(&format!("{BACKEND_URL}/submissions/{submission_id}")).await
}
