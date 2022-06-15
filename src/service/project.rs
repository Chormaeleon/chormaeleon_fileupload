use chrono::NaiveDateTime;

use crate::{
    components::project_list::Project,
    pages::{home::CreateProjectBody, project::ProjectTo},
    utilities::requests::fetch::{
        delete_request, get_request_string, get_request_struct, post_request_struct, FetchError,
    },
};

pub async fn delete_project(project_id: i32) -> Result<(), FetchError> {
    delete_request(&format!("http://localhost:8001/projects/{project_id}")).await
}

pub async fn project_data(project_id: i32) -> Result<ProjectTo, FetchError> {
    get_request_struct::<ProjectTo>(format!("http://localhost:8001/projects/{}", project_id)).await
}

pub fn all_submissions_link(project_id: i32, key: String) -> String {
    format!("http://localhost:8001/projects/{project_id}/allSubmissions?jwt={key}")
}

pub async fn all_submissions_download_key(project_id: i32) -> Result<String, FetchError> {
    get_request_string(format!(
        "http://localhost:8001/projects/{project_id}/downloadKey"
    ))
    .await
}

pub fn submission_upload_url(project_id: i32) -> String {
    format!("http://localhost:8001/projects/{project_id}")
}

pub async fn get_pending_projects() -> Result<Vec<Project>, FetchError> {
    get_request_struct::<Vec<Project>>("http://localhost:8001/pendingProjects".to_string()).await
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

    post_request_struct::<CreateProjectBody, Project>("http://localhost:8001/projects", body).await
}
