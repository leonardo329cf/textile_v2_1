use std::{time::Duration, path};

use tauri::{State, api::path::home_dir};
use tokio::time::sleep;

use crate::{models::app_error::{AppError, DEFAULT_ERROR_CODE}, CutDispositionInputState, services::file_service::{GENERATED_FILES_FOLDER, DISPOSITION_FOLDER, write_to_new_file}};

#[tauri::command]
pub async fn export_disposition(file_name: String, state: State<'_, CutDispositionInputState>) -> Result<String, AppError> {
    if file_name.trim() == "" {
        return Err(AppError::new(1, format!("Nome inválido: {}", file_name).as_str()));
    }

    sleep(Duration::from_millis(1)).await;

    let disposition_json = serde_json::to_string(state.cut_disposition_state.as_ref())
    .map_err(|_| AppError::new(DEFAULT_ERROR_CODE, "Falha ao salvar disposição"))?;

    let mut home_path = DISPOSITION_FOLDER.to_string();
    if let Some(home_path_buf) = home_dir() {
        if let Some(home_str) = home_path_buf.to_str() {
            home_path = format!("{}{}{}{}{}{}{}", 
            home_str, 
            path::MAIN_SEPARATOR_STR,
            GENERATED_FILES_FOLDER,
            path::MAIN_SEPARATOR_STR,
            DISPOSITION_FOLDER,
            path::MAIN_SEPARATOR_STR,
            file_name);
        }
    }
    write_to_new_file(&home_path, &disposition_json).await
    .map_err(|e| AppError::new(DEFAULT_ERROR_CODE, e.to_string().as_str()))
}