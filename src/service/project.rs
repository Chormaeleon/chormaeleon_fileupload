use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{
    components::project_list::Project,
    pages::home::CreateProjectBody,
    utilities::requests::fetch::{
        delete_request, get_request_string, get_request_struct, post_request_struct, FetchError,
    },
};

use super::BACKEND_URL;

pub async fn delete_project(project_id: i32) -> Result<(), FetchError> {
    delete_request(&format!("{BACKEND_URL}/projects/{project_id}")).await
}

pub async fn project_data(project_id: i32) -> Result<ProjectTo, FetchError> {
    get_request_struct::<ProjectTo>(&format!("{BACKEND_URL}/projects/{}", project_id)).await
}

pub fn all_submissions_link(project_id: i32, key: String) -> String {
    format!("{BACKEND_URL}/projects/{project_id}/allSubmissions?jwt={key}")
}

pub async fn all_submissions_download_key(project_id: i32) -> Result<String, FetchError> {
    get_request_string(format!("{BACKEND_URL}/projects/{project_id}/downloadKey")).await
}

pub fn submission_upload_url(project_id: i32) -> String {
    format!("{BACKEND_URL}/projects/{project_id}")
}

pub async fn get_pending_projects() -> Result<Vec<Project>, FetchError> {
    get_request_struct::<Vec<Project>>(&format!("{BACKEND_URL}/pendingProjects")).await
}

pub async fn create_project(
    title: String,
    description: String,
    due_date: NaiveDateTime,
) -> Result<Project, FetchError> {
    let body = CreateProjectBody {
        title,
        description,
        due_date,
    };

    post_request_struct::<CreateProjectBody, Project>(&format!("{BACKEND_URL}/projects"), body)
        .await
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct ProjectTo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub creator: i32,
    pub created_at: NaiveDateTime,
    pub due: NaiveDateTime,
}
