use std::{time::Duration, path::{self}};

use tauri::{State, api::{path::home_dir, dialog::blocking::FileDialogBuilder}};
use tokio::time::sleep;

use crate::{models::{app_error::{AppError, DEFAULT_ERROR_CODE}, cut_disposition::CutDispositionState}, CutDispositionInputState, services::file_service::{GENERATED_FILES_FOLDER, DISPOSITION_FOLDER, write_to_new_file, get_file_text}};

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

#[tauri::command]
pub async fn import_disposition(state: State<'_, CutDispositionInputState>) -> Result<String, AppError> {
    let mut home_path = "".to_string();
    if let Some(home_path_buf) = home_dir() {
        if let Some(home_str) = home_path_buf.to_str() {
            home_path = format!("{}{}{}{}{}", 
            home_str, 
            path::MAIN_SEPARATOR_STR,
            GENERATED_FILES_FOLDER,
            path::MAIN_SEPARATOR_STR,
            DISPOSITION_FOLDER);
        }
    }
    
    let file_path = FileDialogBuilder::new().set_directory(home_path).pick_file();
    if let Some(path_buf) = file_path {
        if let Some(path) = path_buf.to_str() {
            let content = get_file_text(path).await
                .map_err(|e| AppError::new(1, &e.to_string()))?;

            let content: CutDispositionState = serde_json::from_str(&content)
                .map_err(|_| AppError::new(1, "Falha ao carregado o arquivo"))?;

            let mut lock = state.cut_disposition_state.lock()
                .map_err(|_| AppError::new(1, "Falha ao carregado o arquivo"))?;

            *lock = content;

            return Ok("Disposição importada".to_string());
        }
    }
    Err(AppError::new(1, "Nenhum arquivo selecionado"))
}