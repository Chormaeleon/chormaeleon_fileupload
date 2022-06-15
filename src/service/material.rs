pub fn material_url(project_id: i32, file_technical_name: &str) -> String {
    format!("http://localhost:8001/materials/{project_id}/{file_technical_name}")
}

pub fn material_upload_url(project_id: i32) -> String {
    format!("http://localhost:8001/projects/{project_id}/material")
}
