use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use yew::{html, ToHtml};

use crate::{
    service::backend,
    utilities::requests::fetch::{
        delete_request, get_request_struct, post_request_struct, FetchError,
    },
};

pub fn material_url(project_id: i64, file_technical_name: &str) -> String {
    let backend_url = backend();
    format!("{backend_url}/materials/{project_id}/{file_technical_name}")
}

pub fn material_upload_url(project_id: i64) -> String {
    let backend_url = backend();
    format!("{backend_url}/projects/{project_id}/material")
}

pub async fn material_by_project(project_id: i64) -> Result<Vec<MaterialTo>, FetchError> {
    let backend_url = backend();
    get_request_struct(&format!("{backend_url}/projects/{project_id}/material")).await
}

pub async fn update_material(
    material_id: i64,
    changes: UpdateMaterial,
) -> Result<MaterialTo, FetchError> {
    let backend_url = backend();
    post_request_struct(&format!("{backend_url}/materials/{material_id}"), changes).await
}

pub async fn delete_material(material_id: i64) -> Result<(), FetchError> {
    let backend_url = backend();
    let url = format!("{backend_url}/materials/{material_id}");
    delete_request(&url).await
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct MaterialTo {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub file_name: String,
    pub file_technical_name: String,
    pub creator: i64,
    pub upload_at: PrimitiveDateTime,
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

impl Display for MaterialCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let category = match self {
            MaterialCategory::Audio => "Audio",
            MaterialCategory::Video => "Video",
            MaterialCategory::SheetMusic => "Noten",
            MaterialCategory::Other => "Sonstiges",
        };

        write!(f, "{}", category)
    }
}

impl ToHtml for MaterialCategory {
    fn to_html(&self) -> yew::Html {
        html!(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct UpdateMaterial {
    pub title: String,
    pub category: MaterialCategory,
}
