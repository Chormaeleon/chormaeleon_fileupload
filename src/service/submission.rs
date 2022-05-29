use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utilities::requests::fetch::{delete_request, get_request_struct, FetchError};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Section {
    Soprano,
    Alto,
    Tenor,
    Bass,
    Conductor,
    Instrument,
}

pub async fn submissions_by_project(project_id: i32) -> Result<Vec<Submission>, FetchError> {
    get_request_struct::<Vec<Submission>>(format!(
        "http://localhost:8001/projects/{}/submissions",
        project_id
    ))
    .await
}

pub async fn submissions_by_project_and_user(
    project_id: i32,
    user_id: i32,
) -> Result<Vec<Submission>, FetchError> {
    get_request_struct::<Vec<Submission>>(format!(
        "http://localhost:8001/projects/{project_id}/submissions/{user_id}"
    ))
    .await
}

pub fn submission_download_url(submission_id: i32) -> String {
    format!("http://localhost:8001/submissions/{submission_id}")
}

pub async fn delete_submission(submission_id: i32) -> Result<(), FetchError> {
    delete_request(&format!(
        "http://localhost:8001/submissions/{submission_id}"
    )).await
}
