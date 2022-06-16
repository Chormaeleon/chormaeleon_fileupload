use super::BACKEND_URL;

pub fn material_url(project_id: i32, file_technical_name: &str) -> String {
    format!("{BACKEND_URL}/materials/{project_id}/{file_technical_name}")
}

pub fn material_upload_url(project_id: i32) -> String {
    format!("{BACKEND_URL}/projects/{project_id}/material")
}
