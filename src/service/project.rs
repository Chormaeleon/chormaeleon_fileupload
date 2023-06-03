use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::utilities::requests::fetch::{
    delete_request, get_request_struct, post_request_struct, FetchError,
};

use super::backend;

pub async fn delete_project(project_id: i64) -> Result<(), FetchError> {
    let backend_url = backend();
    delete_request(&format!("{backend_url}/projects/{project_id}")).await
}

pub async fn project_data(project_id: i64) -> Result<ProjectTo, FetchError> {
    let backend_url = backend();
    get_request_struct::<ProjectTo>(&format!("{backend_url}/projects/{}", project_id)).await
}

pub fn all_submissions_link(project_id: i64) -> String {
    let backend_url = backend();
    format!("{backend_url}/projects/{project_id}/allSubmissions")
}

pub fn submission_upload_url(project_id: i64) -> String {
    let backend_url = backend();
    format!("{backend_url}/projects/{project_id}")
}

pub async fn get_pending_projects() -> Result<Vec<ProjectTo>, FetchError> {
    let backend_url = backend();
    get_request_struct::<Vec<ProjectTo>>(&format!("{backend_url}/projects/pending")).await
}

pub async fn get_my_projects() -> Result<Vec<ProjectTo>, FetchError> {
    let backend_url = backend();
    get_request_struct::<Vec<ProjectTo>>(&format!("{backend_url}/projects/myProjects")).await
}
pub async fn get_all_projects() -> Result<Vec<ProjectTo>, FetchError> {
    let backend_url = backend();
    get_request_struct::<Vec<ProjectTo>>(&format!("{backend_url}/projects/all")).await
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

    let backend_url = backend();

    post_request_struct::<CreateProjectBody, ProjectTo>(&format!("{backend_url}/projects"), body)
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
    let backend_url = backend();
    post_request_struct::<UpdateProject, ProjectTo>(
        &format!("{backend_url}/projects/{project_id}"),
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
