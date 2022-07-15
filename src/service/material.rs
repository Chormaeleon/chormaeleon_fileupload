use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::utilities::requests::fetch::{delete_request, FetchError};

use super::BACKEND_URL;

pub fn material_url(project_id: i32, file_technical_name: &str) -> String {
    format!("{BACKEND_URL}/materials/{project_id}/{file_technical_name}")
}

pub fn material_upload_url(project_id: i32) -> String {
    format!("{BACKEND_URL}/projects/{project_id}/material")
}

pub async fn delete_material(material_id: i32) -> Result<(), FetchError> {
    let url = format!("{BACKEND_URL}/materials/{material_id}");
    delete_request(&url).await
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct MetadataEntry {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub file_name: String,
    pub file_technical_name: String,
    pub creator: i32,
    pub upload_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum MaterialKind {
    Audio,
    Video,
    SheetMusic,
    Other,
}
