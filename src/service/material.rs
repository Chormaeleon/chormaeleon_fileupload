use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utilities::requests::fetch::{delete_request, get_request_struct, FetchError};

use super::BACKEND_URL;

pub fn material_url(project_id: i32, file_technical_name: &str) -> String {
    format!("{BACKEND_URL}/materials/{project_id}/{file_technical_name}")
}

pub fn material_upload_url(project_id: i32) -> String {
    format!("{BACKEND_URL}/projects/{project_id}/material")
}

pub async fn material_by_project(project_id: i32) -> Result<Vec<MaterialTo>, FetchError> {
    get_request_struct(format!("{BACKEND_URL}/projects/{project_id}/material")).await
}

pub async fn delete_material(material_id: i32) -> Result<(), FetchError> {
    let url = format!("{BACKEND_URL}/materials/{material_id}");
    delete_request(&url).await
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MaterialTo {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub file_name: String,
    pub file_technical_name: String,
    pub creator: i32,
    pub upload_at: NaiveDateTime,
    pub category: MaterialCategory,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MaterialCategory {
    Audio,
    Video,
    SheetMusic,
    Other,
}
