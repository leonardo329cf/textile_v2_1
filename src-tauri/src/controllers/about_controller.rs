use crate::{models::app_error::{AppError, DEFAULT_ERROR_CODE}, services::about_service::{get_about_text, AboutServiceError}};


#[tauri::command]
pub async fn get_about() -> Result<String, AppError> {
    get_about_text().await.map_err(|error| match error {
        AboutServiceError::FailedToReadFile { path } => AppError::new(
            DEFAULT_ERROR_CODE,
            format!("Falha ao abrir ou ler arquivo localizado em: {}", path).as_str(),
        ),
    })
}
