use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::utilities::requests::fetch::{
    delete_request, get_request_struct, post_request_struct, FetchError,
};

use super::BACKEND_URL;

pub async fn delete_project(project_id: i64) -> Result<(), FetchError> {
    delete_request(&format!("{BACKEND_URL}/projects/{project_id}")).await
}

pub async fn project_data(project_id: i64) -> Result<ProjectTo, FetchError> {
    get_request_struct::<ProjectTo>(&format!("{BACKEND_URL}/projects/{}", project_id)).await
}

pub fn all_submissions_link(project_id: i64) -> String {
    format!("{BACKEND_URL}/projects/{project_id}/allSubmissions")
}

pub fn submission_upload_url(project_id: i64) -> String {
    format!("{BACKEND_URL}/projects/{project_id}")
}

pub async fn get_pending_projects() -> Result<Vec<ProjectTo>, FetchError> {
    get_request_struct::<Vec<ProjectTo>>(&format!("{BACKEND_URL}/projects/pending")).await
}

pub async fn get_my_projects() -> Result<Vec<ProjectTo>, FetchError> {
    get_request_struct::<Vec<ProjectTo>>(&format!("{BACKEND_URL}/projects/myProjects")).await
}
pub async fn get_all_projects() -> Result<Vec<ProjectTo>, FetchError> {
    get_request_struct::<Vec<ProjectTo>>(&format!("{BACKEND_URL}/projects/all")).await
}

pub async fn create_project(
    title: String,
    description: String,
    due_date: PrimitiveDateTime,
) -> Result<ProjectTo, FetchError> {
    let body = CreateProjectBody {
        title,
        description,
        due_date,
    };

    post_request_struct::<CreateProjectBody, ProjectTo>(&format!("{BACKEND_URL}/projects"), body)
        .await
}

pub async fn update_project(
    project_id: i64,
    title: String,
    description: String,
    due: PrimitiveDateTime,
) -> Result<ProjectTo, FetchError> {
    let body = UpdateProject {
        title,
        description,
        due,
    };

    post_request_struct::<UpdateProject, ProjectTo>(
        &format!("{BACKEND_URL}/projects/{project_id}"),
        body,
    )
    .await
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ProjectTo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub creator: i64,
    pub created_at: PrimitiveDateTime,
    pub due: PrimitiveDateTime,
}

#[derive(Clone, Serialize)]
pub struct CreateProjectBody {
    pub title: String,
    pub description: String,
    pub due_date: PrimitiveDateTime,
}

#[derive(Clone, Serialize)]
pub struct UpdateProject {
    pub title: String,
    pub description: String,
    pub due: PrimitiveDateTime,
}
