use crate::utilities::requests::fetch::{delete_request, FetchError};

pub async fn delete_project(project_id: i32) -> Result<(), FetchError> {
    delete_request(&format!("http://localhost:8001/projects/{project_id}")).await
}
