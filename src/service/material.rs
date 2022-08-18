use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utilities::requests::fetch::{
    delete_request, get_request_struct, post_request_struct, FetchError,
};

use super::BACKEND_URL;

pub fn material_url(project_id: i32, file_technical_name: &str) -> String {
    format!("{BACKEND_URL}/materials/{project_id}/{file_technical_name}")
}

pub fn material_upload_url(project_id: i32) -> String {
    format!("{BACKEND_URL}/projects/{project_id}/material")
}

pub async fn material_by_project(project_id: i32) -> Result<Vec<MaterialTo>, FetchError> {
    get_request_struct(&format!("{BACKEND_URL}/projects/{project_id}/material")).await
}

pub async fn update_material(
    material_id: i32,
    changes: UpdateMaterial,
) -> Result<MaterialTo, FetchError> {
    post_request_struct(&format!("{BACKEND_URL}/materials/{material_id}"), changes).await
}

pub async fn delete_material(material_id: i32) -> Result<(), FetchError> {
    let url = format!("{BACKEND_URL}/materials/{material_id}");
    delete_request(&url).await
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum MaterialCategory {
    Audio,
    Video,
    SheetMusic,
    Other,
}

impl TryFrom<&str> for MaterialCategory {
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let result = match string {
            "Audio" | "audio" => MaterialCategory::Audio,
            "Video" | "video" => MaterialCategory::Video,
            "Sheet" | "sheet" | "sheetMusic" | "sheetmusic" => MaterialCategory::SheetMusic,
            "Other" | "other" => MaterialCategory::Other,
            _ => return Err(()),
        };

        Ok(result)
    }

    type Error = ();
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct UpdateMaterial {
    pub title: String,
    pub category: MaterialCategory,
}
